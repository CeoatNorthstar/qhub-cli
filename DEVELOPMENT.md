# QHub Authentication & Database Strategy

## Overview

QHub uses a **dual-branch, dual-database strategy** for enterprise-grade development and deployment:

- **main branch** → Cloudflare D1 (Production)
- **staging branch** → PostgreSQL (Local/Testing)

## Branch Strategy

### main (Production)
- **Database**: Cloudflare D1 (serverless SQLite)
- **Purpose**: Production-ready code only
- **Deployment**: Cloudflare Workers
- **Environment**: `.env.production` (not committed)
- **Protection**: Requires PR reviews
- **.gitignore**: Strict - blocks all `.env*` files

### staging (Development/Testing)
- **Database**: PostgreSQL via Docker
- **Purpose**: Integration testing, team collaboration  
- **Environment**: `.env.local` (committed for team sharing)
- **Protection**: Less strict, allows direct pushes
- **.gitignore**: Relaxed - allows `.env.local`

## Quick Start

### Local Development on Staging

```bash
# Clone and switch to staging
git clone <repo>
cd qhub-cli
git checkout staging

# Start PostgreSQL
docker-compose up -d

# Environment already configured in .env.local
# Just run the app
cargo run
```

### Working on a Feature

```bash
# Create feature branch from staging
git checkout staging
git pull origin staging
git checkout -b feature/my-feature

# Develop and test locally
docker-compose up -d
cargo run

# Commit changes
git add .
git commit -m "feat: my awesome feature"

# Merge to staging for team testing
git checkout staging
git merge feature/my-feature
git push origin staging

# After testing, merge staging to main
git checkout main
git merge staging --no-commit
# Review changes, ensure no .env.local slipped in
git commit -m "feat: merge tested features from staging"
git push origin main
```

## Environment Files

| File | Branch | Committed? | Purpose |
|------|--------|-----------|---------|
| `.env.example` | Both | ✅ Yes | Template, no secrets |
| `.env.production` | main | ✅ Yes | Template for prod (placeholder values) |
| `.env.local` | staging | ✅ Yes | Real dev credentials, shared with team |
| `.env` | Both | ❌ No | Your personal overrides |

## Database Setup

### Staging (PostgreSQL)

```bash
# On staging branch
docker-compose up -d
cargo run  # Migrations run automatically
```

### Production (Cloudflare D1)

```bash
# Create D1 database
wrangler d1 create qhub-production

# Apply migrations
wrangler d1 execute qhub-production --file=./migrations/001_init_schema_d1.sql

# Deploy
wrangler deploy
```

## Security Model

### Staging Branch Security
- ✅ Safe for dev credentials
- ✅ PostgreSQL in Docker (isolated)
- ✅ Never exposed to public
- ✅ Team can share safely via `.env.local`
- ⚠️ Don't put production secrets here

### Main Branch Security
- 🔒 No committed secrets ever
- 🔒 `.env.production` has placeholders only
- 🔒 Real prod secrets in CI/CD or Cloudflare Workers
- 🔒 Strict `.gitignore` prevents accidents

## Authentication Flow

1. User registers/logs in via CLI
2. Password hashed with Argon2
3. JWT token generated
4. Token stored in `~/.qhub/config.toml`
5. Token sent with API requests
6. Server validates JWT + session

## API Architecture

```
┌─────────────┐
│   CLI TUI   │ ←→ JWT Token
└──────┬──────┘
       │
       ↓
┌──────────────────────┐
│  Auth Service (Rust) │ ←→ Verify token, sessions
└──────┬───────────────┘
       │
       ↓
┌──────────────────────┐
│ Database             │
│ • Staging: Postgres  │
│ • Prod: D1 (SQLite)  │
└──────────────────────┘
```

## Database Schema Compatibility

Both PostgreSQL and Cloudflare D1 use the same logical schema:

- `users` - User accounts
- `user_sessions` - Active sessions
- `oauth_connections` - OAuth providers
- `api_keys` - API tokens
- `user_preferences` - Settings
- `usage_records` - Quotas
- `quantum_jobs` - Job history

**Differences**:
- PostgreSQL: UUID types, JSONB, triggers
- D1 (SQLite): TEXT for UUIDs, TEXT for JSON, simplified triggers

## Development Workflow

### Day-to-Day Development

```bash
# Always work on staging
git checkout staging
git pull origin staging

# Start database
docker-compose up -d

# Make changes
cargo run
cargo test

# Commit to staging
git add .
git commit -m "feat: my change"
git push origin staging
```

### Release to Production

```bash
# Merge tested code from staging
git checkout main
git pull origin main
git merge staging --no-commit

# IMPORTANT: Review changes
git status
git diff --staged

# Ensure no .env.local made it in
git reset HEAD .env.local  # if it appears

# Commit and deploy
git commit -m "release: v0.2.0"
git push origin main

# Deploy to Cloudflare
wrangler deploy
```

## Troubleshooting

### "Cannot connect to database"
```bash
# On staging: Check Docker
docker ps | grep qhub-postgres
docker-compose restart

# On main: Check D1 connection
wrangler d1 info qhub-production
```

### "JWT_SECRET not set"
```bash
# On staging: Check .env.local exists
cat .env.local | grep JWT_SECRET

# On main: Set in Cloudflare Workers dashboard
```

### "Migration failed"
```bash
# Check current version
sqlx migrate info

# Revert and retry
sqlx migrate revert
sqlx migrate run
```

## Best Practices

1. ✅ **Always develop on staging branch**
2. ✅ **Test locally before pushing to staging**
3. ✅ **Test on staging before merging to main**
4. ✅ **Review diffs when merging staging → main**
5. ✅ **Keep .env.local updated on staging**
6. ❌ **Never commit production secrets**
7. ❌ **Never edit .gitignore on staging to allow more files**
8. ❌ **Never develop directly on main**

## CI/CD Integration

### Staging Pipeline
```yaml
on: push
  branches: [staging]
jobs:
  test:
    - Start PostgreSQL
    - Run migrations
    - Run tests
    - Run integration tests
```

### Production Pipeline
```yaml
on: push
  branches: [main]
jobs:
  deploy:
    - Build release
    - Run migrations on D1
    - Deploy to Cloudflare Workers
    - Run smoke tests
```

## Team Collaboration

### Onboarding New Developers

```bash
# 1. Clone repo
git clone <repo>
cd qhub-cli

# 2. Switch to staging
git checkout staging

# 3. Start database
docker-compose up -d

# 4. Run app (.env.local already configured!)
cargo run
```

### Sharing Database Changes

```bash
# Create migration
sqlx migrate add add_feature_x

# Test locally
sqlx migrate run
cargo test

# Commit to staging (includes .env.local)
git add migrations/
git commit -m "feat: add feature X schema"
git push origin staging

# Team pulls and runs
git pull origin staging
sqlx migrate run
```

## Support

- Database issues: See `DATABASE.md`
- Configuration: See `CONFIG.md`
- General setup: See `README.md`

## Security Contacts

If you discover a security issue:
- **Do not** create a public GitHub issue
- Email: security@qhub.dev
- Include "SECURITY" in subject line
