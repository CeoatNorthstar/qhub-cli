# QHub Workers - Development Guide

Comprehensive guide for developing and maintaining the QHub API backend.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Database Management](#database-management)
- [Authentication Flow](#authentication-flow)
- [Adding New Features](#adding-new-features)
- [Testing](#testing)
- [Deployment](#deployment)
- [Monitoring & Debugging](#monitoring--debugging)
- [Best Practices](#best-practices)

## Architecture Overview

### Tech Stack

- **Runtime**: Cloudflare Workers (Edge computing)
- **Framework**: Hono (Fast, lightweight web framework)
- **Database**: D1 (Cloudflare's serverless SQL database)
- **AI**: Cloudflare AI (Llama-2-7B model)
- **Authentication**: JWT with bcrypt
- **Language**: TypeScript

### Key Components

```
┌─────────────┐
│   Client    │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────┐
│   Cloudflare Edge Network       │
│  ┌───────────────────────────┐  │
│  │   QHub Worker (Hono)      │  │
│  │  ┌────────────────────┐   │  │
│  │  │ Auth Middleware    │   │  │
│  │  └────────────────────┘   │  │
│  │  ┌────────────────────┐   │  │
│  │  │ Route Handlers     │   │  │
│  │  │ - Auth             │   │  │
│  │  │ - AI Chat          │   │  │
│  │  │ - Quantum Jobs     │   │  │
│  │  └────────────────────┘   │  │
│  └───────────┬─────────────────┘│
└──────────────┼──────────────────┘
               │
       ┌───────┴────────┐
       ▼                ▼
  ┌─────────┐    ┌──────────┐
  │   D1    │    │  CF AI   │
  │Database │    │  Service │
  └─────────┘    └──────────┘
```

## Development Setup

### Initial Setup

1. **Install dependencies**:
   ```bash
   cd workers
   npm install
   ```

2. **Login to Cloudflare**:
   ```bash
   wrangler login
   ```

3. **Create databases**:
   ```bash
   # Development
   wrangler d1 create qhub-dev
   
   # Staging (optional)
   wrangler d1 create qhub-staging
   
   # Production
   wrangler d1 create qhub-production
   ```

4. **Update wrangler.toml** with database IDs returned from step 3.

5. **Run migrations**:
   ```bash
   npm run db:migrate:dev
   ```

6. **Set secrets**:
   ```bash
   # Local development (add to .dev.vars)
   echo 'JWT_SECRET="your-super-secret-key-change-this-in-production-32chars"' > .dev.vars
   echo 'JWT_EXPIRY_HOURS="24"' >> .dev.vars
   echo 'ENVIRONMENT="development"' >> .dev.vars
   
   # For remote environments
   wrangler secret put JWT_SECRET --env staging
   wrangler secret put JWT_SECRET --env production
   ```

### Running Locally

```bash
# Start dev server with hot reload
npm run dev

# The API will be available at http://localhost:8787
```

## Project Structure

```
workers/
├── src/
│   ├── index.ts              # Main entry point, route mounting
│   ├── types.ts              # TypeScript interfaces and types
│   ├── utils.ts              # Utility functions (crypto, JWT, etc.)
│   ├── middleware/
│   │   └── auth.ts           # JWT authentication middleware
│   └── routes/
│       ├── auth.ts           # Authentication endpoints
│       ├── ai.ts             # AI chat endpoints
│       └── quantum.ts        # Quantum job endpoints
├── migrations/
│   └── 001_init_schema.sql   # Database schema definition
├── package.json              # Dependencies and scripts
├── tsconfig.json             # TypeScript configuration
├── wrangler.toml             # Cloudflare Workers config
├── README.md                 # User documentation
├── DEVELOPMENT.md            # This file
└── API_EXAMPLES.md           # API testing examples
```

### File Responsibilities

- **index.ts**: App initialization, middleware setup, route mounting
- **types.ts**: All TypeScript type definitions
- **utils.ts**: Shared utility functions
- **middleware/auth.ts**: Request authentication and authorization
- **routes/*.ts**: API endpoint implementations

## Database Management

### Schema Design

The database uses SQLite (via D1) with the following tables:

- `users` - User accounts
- `user_sessions` - Login sessions with JWT tokens
- `api_keys` - API keys for programmatic access (future)
- `conversations` - AI chat conversations
- `messages` - Individual chat messages
- `quantum_jobs` - Quantum circuit job submissions
- `usage_records` - Usage tracking for billing (future)

### Running Migrations

```bash
# Local (development)
npm run db:migrate:dev

# Staging
npm run db:migrate:staging

# Production
npm run db:migrate:production
```

### Querying the Database

```bash
# Development (local)
npm run db:query:dev -- "SELECT * FROM users LIMIT 5"

# Staging
npm run db:query:staging -- "SELECT COUNT(*) FROM users"

# Production
npm run db:query:production -- "SELECT * FROM quantum_jobs WHERE status='pending'"
```

### Database Backups

```bash
# Export database
wrangler d1 export qhub-production --output=backup-$(date +%Y%m%d).sql

# Import database
wrangler d1 execute qhub-production --file=backup-20240115.sql
```

### Adding New Tables

1. Create a new migration file: `migrations/002_your_feature.sql`
2. Write SQL DDL statements
3. Run the migration
4. Add TypeScript types to `types.ts`
5. Update queries in route handlers

Example migration:
```sql
-- migrations/002_add_teams.sql
CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_teams_name ON teams(name);
```

## Authentication Flow

### Registration

1. Client sends `POST /auth/register` with email, password, username
2. Server validates input
3. Password is hashed with bcrypt (10 rounds)
4. User record created in database
5. JWT token generated and signed
6. Session record created with hashed token
7. Token returned to client

### Login

1. Client sends `POST /auth/login` with email, password
2. Server fetches user by email
3. Password verified against stored hash
4. JWT token generated
5. New session created
6. Token returned to client

### Protected Routes

1. Client sends request with `Authorization: Bearer <token>`
2. Auth middleware extracts token
3. Token verified and decoded
4. User fetched from database
5. User attached to context
6. Request proceeds to handler

## Adding New Features

### Adding a New Route

1. **Create route file**: `src/routes/yourfeature.ts`

```typescript
import { Hono } from 'hono';
import { Env, Variables } from '../types';
import { authMiddleware } from '../middleware/auth';

const yourfeature = new Hono<{ Bindings: Env; Variables: Variables }>();

yourfeature.get('/hello', authMiddleware, async (c) => {
  const user = c.get('user');
  return c.json({ message: `Hello, ${user.email}!` });
});

export default yourfeature;
```

2. **Mount in index.ts**:

```typescript
import yourfeature from './routes/yourfeature';
app.route('/yourfeature', yourfeature);
```

3. **Add types** to `types.ts` if needed

4. **Test** the endpoint

### Adding Middleware

Create middleware in `src/middleware/`:

```typescript
import { Context, Next } from 'hono';
import { Env, Variables } from '../types';

export async function ratelimitMiddleware(
  c: Context<{ Bindings: Env; Variables: Variables }>, 
  next: Next
) {
  // Check rate limit
  const rateLimitKey = `ratelimit:${c.get('user').id}`;
  // ... implementation
  
  await next();
}
```

Use in routes:
```typescript
route.post('/endpoint', authMiddleware, ratelimitMiddleware, handler);
```

## Testing

### Manual Testing

Use the examples in `API_EXAMPLES.md` or tools like:

- cURL
- Postman
- Insomnia
- HTTPie

### Automated Testing

```bash
npm test
```

Create test files in `src/__tests__/`:

```typescript
// src/__tests__/auth.test.ts
import { describe, it, expect } from 'vitest';
// Add test implementations
```

### Testing Checklist

- [ ] All endpoints return correct status codes
- [ ] Authentication works correctly
- [ ] Error handling returns proper messages
- [ ] Input validation works
- [ ] Database queries are correct
- [ ] Usage limits are enforced
- [ ] CORS headers are present

## Deployment

### Staging Deployment

```bash
# Deploy to staging
npm run deploy:staging

# Test staging
curl https://qhub-api-staging.workers.dev/health
```

### Production Deployment

```bash
# Deploy to production
npm run deploy:production

# Verify deployment
curl https://qhub-api.workers.dev/health
```

### Deployment Checklist

- [ ] All tests pass locally
- [ ] Database migrations run successfully
- [ ] Secrets are set in environment
- [ ] Environment variables configured
- [ ] Staging deployment tested
- [ ] Production deployment coordinated
- [ ] Monitoring alerts configured
- [ ] Documentation updated

### Rollback

If deployment fails:

```bash
# List deployments
wrangler deployments list

# Rollback to previous version
wrangler rollback [deployment-id]
```

## Monitoring & Debugging

### View Logs

```bash
# Real-time logs
wrangler tail

# Filter by status
wrangler tail --status error

# Filter by method
wrangler tail --method POST
```

### Cloudflare Dashboard

Monitor in dashboard:
- Request volume
- Error rates
- Response times
- Geographic distribution

### Debugging Tips

1. **Add console.log statements**: They appear in `wrangler tail`

2. **Check environment variables**:
   ```typescript
   console.log('Environment:', c.env.ENVIRONMENT);
   ```

3. **Inspect request details**:
   ```typescript
   console.log('Headers:', c.req.header());
   console.log('Body:', await c.req.json());
   ```

4. **Test locally first**: Always test with `npm run dev` before deploying

5. **Check database state**:
   ```bash
   npm run db:query:dev -- "SELECT * FROM users WHERE email='test@example.com'"
   ```

## Best Practices

### Code Style

- Use TypeScript strict mode
- Add JSDoc comments for complex functions
- Keep functions small and focused
- Use meaningful variable names
- Follow existing patterns

### Security

- Never log sensitive data (passwords, tokens)
- Always validate and sanitize input
- Use parameterized queries (prevent SQL injection)
- Set appropriate CORS policies
- Keep dependencies updated
- Use secrets for sensitive config

### Performance

- Minimize database queries
- Use indexes on frequently queried fields
- Cache when appropriate
- Keep response payloads small
- Use pagination for lists

### Database

- Always use transactions for multi-step operations
- Add indexes for foreign keys and search fields
- Use `LIMIT` on queries that could return many rows
- Clean up expired sessions periodically

### Error Handling

- Return appropriate HTTP status codes
- Provide clear error messages
- Log errors for debugging
- Don't expose internal details in production

### API Design

- Use RESTful conventions
- Version your API if making breaking changes
- Document all endpoints
- Return consistent JSON structures
- Use proper HTTP methods

## Common Tasks

### Adding a New Tier

1. Update usage limits in route files:
   ```typescript
   const USAGE_LIMITS: Record<string, number> = {
     free: 10,
     pro: 100,
     premium: 500,  // New tier
     enterprise: 1000
   };
   ```

2. Update tier checks in middleware if needed

3. Update documentation

### Cleaning Up Old Sessions

Add a scheduled worker (future enhancement):

```typescript
// In wrangler.toml
[triggers]
crons = ["0 0 * * *"]  # Daily at midnight

// In index.ts
export default {
  async scheduled(event: ScheduledEvent, env: Env) {
    const now = Math.floor(Date.now() / 1000);
    await env.DB.prepare(
      'DELETE FROM user_sessions WHERE expires_at < ?'
    ).bind(now).run();
  }
}
```

### Adding Email Verification

1. Add `email_verification_token` to users table
2. Create `/auth/verify-email/:token` endpoint
3. Send email with verification link
4. Update registration to set `email_verified = 0`
5. Check `email_verified` in protected routes if required

### Implementing Rate Limiting

Consider using Cloudflare Rate Limiting or Durable Objects:

```typescript
// Using Durable Objects for rate limiting
export class RateLimiter {
  async fetch(request: Request) {
    const key = request.headers.get('CF-Connecting-IP');
    // Implement token bucket or sliding window
  }
}
```

## Troubleshooting

### Common Issues

1. **"Database not found"**
   - Ensure database is created: `wrangler d1 list`
   - Check database ID in wrangler.toml
   - Run migrations

2. **"JWT verification failed"**
   - Check JWT_SECRET is set correctly
   - Verify token hasn't expired
   - Ensure clock sync

3. **"AI binding not found"**
   - Verify AI binding in wrangler.toml
   - Check account has AI enabled

4. **"CORS errors"**
   - Check CORS middleware configuration
   - Verify origin is allowed
   - Check preflight OPTIONS handling

5. **"Database locked"**
   - D1 has row-level locking
   - Use transactions carefully
   - Avoid long-running queries

## Resources

- [Cloudflare Workers Docs](https://developers.cloudflare.com/workers/)
- [Hono Documentation](https://hono.dev/)
- [D1 Documentation](https://developers.cloudflare.com/d1/)
- [Workers AI Documentation](https://developers.cloudflare.com/workers-ai/)

## Support

For questions or issues:
- Check existing documentation
- Review Cloudflare Workers docs
- Open an issue in the repository
- Contact the development team

---

Happy coding! 🚀
