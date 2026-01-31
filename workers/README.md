# QHub API - TypeScript Backend

Enterprise-grade API backend for QHub, built on Cloudflare Workers with TypeScript, Hono, D1 Database, and Cloudflare AI.

## 🚀 Features

- **Authentication**: JWT-based auth with bcrypt password hashing
- **AI Chat**: Powered by Cloudflare AI (Llama-2-7B)
- **Quantum Jobs**: Quantum circuit job management (IBM Qiskit integration ready)
- **D1 Database**: Serverless SQL database for data persistence
- **Type-Safe**: Full TypeScript support with comprehensive types
- **Edge Computing**: Deploy globally on Cloudflare's edge network
- **Rate Limiting**: Built-in usage limits by user tier
- **Session Management**: Multi-device session tracking

## 📁 Project Structure

```
workers/
├── src/
│   ├── index.ts              # Main application entry point
│   ├── types.ts              # TypeScript type definitions
│   ├── utils.ts              # Utility functions (hashing, JWT, etc.)
│   ├── middleware/
│   │   └── auth.ts           # Authentication middleware
│   └── routes/
│       ├── auth.ts           # Authentication endpoints
│       ├── ai.ts             # AI chat endpoints
│       └── quantum.ts        # Quantum job endpoints
├── migrations/
│   └── 001_init_schema.sql   # Database schema
├── package.json              # Dependencies and scripts
├── tsconfig.json             # TypeScript configuration
├── wrangler.toml             # Cloudflare Workers configuration
└── README.md                 # This file
```

## 🛠️ Setup

### Prerequisites

- Node.js 18+ and npm
- Cloudflare account
- Wrangler CLI (`npm install -g wrangler`)

### Installation

1. **Install dependencies**:
   ```bash
   cd workers
   npm install
   ```

2. **Configure Wrangler**:
   - Login to Cloudflare:
     ```bash
     wrangler login
     ```
   
   - Update `wrangler.toml` with your account ID

3. **Create D1 Database**:
   ```bash
   # Development database
   wrangler d1 create qhub-dev
   
   # Production database (optional)
   wrangler d1 create qhub-production
   ```
   
   Update `wrangler.toml` with the database IDs returned

4. **Run migrations**:
   ```bash
   # Local development
   npm run db:migrate:dev
   
   # Production
   npm run db:migrate:production
   ```

5. **Set secrets**:
   ```bash
   # Generate a secure JWT secret
   wrangler secret put JWT_SECRET
   # Enter a random 32+ character string when prompted
   ```

## 🏃 Development

### Local Development

Start the development server with local D1 database:

```bash
npm run dev
```

The API will be available at `http://localhost:8787`

### Testing Endpoints

```bash
# Health check
curl http://localhost:8787/health

# Register user
curl -X POST http://localhost:8787/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password123"}'

# Login
curl -X POST http://localhost:8787/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password123"}'

# Chat (requires token)
curl -X POST http://localhost:8787/ai/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{"message":"Hello, how are you?"}'
```

## 📡 API Endpoints

### Authentication (`/auth`)

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/auth/register` | Register new user | ❌ |
| POST | `/auth/login` | Login with credentials | ❌ |
| POST | `/auth/logout` | Logout current session | ✅ |
| POST | `/auth/logout-all` | Logout all sessions | ✅ |
| GET | `/auth/verify` | Verify token and get user info | ✅ |
| GET | `/auth/sessions` | List active sessions | ✅ |
| DELETE | `/auth/sessions/:id` | Delete specific session | ✅ |

### AI Chat (`/ai`)

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/ai/chat` | Send message and get AI response | ✅ |
| GET | `/ai/conversations` | List all conversations | ✅ |
| GET | `/ai/conversations/:id` | Get conversation with messages | ✅ |
| DELETE | `/ai/conversations/:id` | Delete conversation | ✅ |
| GET | `/ai/usage` | Get usage statistics | ✅ |

### Quantum Jobs (`/quantum`)

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/quantum/submit` | Submit quantum circuit job | ✅ |
| GET | `/quantum/jobs` | List all jobs (with filters) | ✅ |
| GET | `/quantum/jobs/:id` | Get job details | ✅ |
| DELETE | `/quantum/jobs/:id` | Cancel/delete job | ✅ |
| GET | `/quantum/stats` | Get job statistics | ✅ |
| POST | `/quantum/jobs/:id/rerun` | Rerun existing job | ✅ |

## 🔒 Authentication

The API uses JWT (JSON Web Tokens) for authentication.

### Request Format

Include the token in the `Authorization` header:

```
Authorization: Bearer <your_jwt_token>
```

### Token Expiration

- Default expiration: 24 hours
- Configurable via `JWT_EXPIRY_HOURS` environment variable

### Session Management

- Multiple concurrent sessions supported
- Each session tracks device info and IP address
- Sessions automatically expire when token expires

## 💾 Database Schema

### Tables

- **users**: User accounts
- **user_sessions**: Active login sessions
- **conversations**: AI chat conversations
- **messages**: Individual chat messages
- **quantum_jobs**: Quantum circuit jobs

See `migrations/001_init_schema.sql` for full schema.

## 🎯 Usage Limits

Limits are enforced based on user tier:

### AI Chat (messages per day)
- Free: 10 messages
- Pro: 100 messages
- Enterprise: 1000 messages

### Quantum Jobs (concurrent jobs)
- Free: 3 jobs
- Pro: 10 jobs
- Enterprise: 50 jobs

## 🌍 Deployment

### Deploy to Staging

```bash
npm run deploy:staging
```

### Deploy to Production

```bash
npm run deploy:production
```

### Environment Variables

Configure in Cloudflare dashboard or via Wrangler:

| Variable | Description | Required |
|----------|-------------|----------|
| `JWT_SECRET` | Secret key for JWT signing | ✅ |
| `JWT_EXPIRY_HOURS` | Token expiration time (default: 24) | ❌ |
| `ENVIRONMENT` | Environment name (dev/staging/prod) | ❌ |

### Secrets Management

```bash
# Set JWT secret
wrangler secret put JWT_SECRET

# List secrets
wrangler secret list

# Delete secret
wrangler secret delete JWT_SECRET
```

## 🔧 Database Management

### Query Database

```bash
# Development (local)
npm run db:query:dev -- "SELECT * FROM users LIMIT 5"

# Production
npm run db:query:production -- "SELECT * FROM users LIMIT 5"
```

### Backup Database

```bash
# Export data
wrangler d1 export qhub-production --output=backup.sql
```

## 📊 Monitoring

### Cloudflare Dashboard

Monitor your Worker in the Cloudflare dashboard:
- Request volume
- Error rates
- Execution time
- Geographic distribution

### Logging

Logs are available in Cloudflare dashboard or via Wrangler:

```bash
wrangler tail
```

## 🧪 Testing

### Manual Testing

Use the provided curl examples or tools like Postman/Insomnia.

### Automated Testing

```bash
npm test
```

## 🐛 Troubleshooting

### Common Issues

1. **Database not found**
   - Ensure D1 database is created and ID is in `wrangler.toml`
   - Run migrations: `npm run db:migrate:dev`

2. **JWT errors**
   - Verify `JWT_SECRET` is set: `wrangler secret list`
   - Check token hasn't expired

3. **CORS issues**
   - CORS is configured to allow all origins by default
   - Modify `src/index.ts` if you need stricter CORS

4. **AI responses failing**
   - Check Cloudflare AI binding is configured in `wrangler.toml`
   - Verify your account has AI enabled

## 🚧 Future Enhancements

- [ ] Email verification
- [ ] Password reset flow
- [ ] Rate limiting middleware
- [ ] Webhook support for quantum job completion
- [ ] Real-time quantum job execution via Durable Objects
- [ ] Integration with IBM Quantum Experience API
- [ ] Support for multiple AI models
- [ ] API key authentication for programmatic access
- [ ] Billing and subscription management
- [ ] Admin dashboard endpoints

## 📄 License

MIT License - see LICENSE file for details

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## 📞 Support

For issues and questions:
- GitHub Issues: [your-repo-url]
- Email: support@qhub.dev
- Docs: [docs-url]

---

Built with ❤️ using Cloudflare Workers, Hono, and TypeScript
