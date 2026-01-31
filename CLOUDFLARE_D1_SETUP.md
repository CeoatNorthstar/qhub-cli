# Cloudflare D1 Production Setup

This guide shows how to configure QHub for production deployment with Cloudflare D1.

## Prerequisites

- Cloudflare account
- Wrangler CLI installed (`npm install -g wrangler`)
- Database ID from Cloudflare dashboard

## Step 1: Install Wrangler

```bash
npm install -g wrangler
wrangler login
```

## Step 2: Database Information

Your Cloudflare D1 database has been created with ID: `a52bc4fb-9185-4185-bb6e-572c3dd3feaf`

## Step 3: Apply Migrations to D1

Run the D1-compatible schema migration:

```bash
wrangler d1 execute qhub-production \
  --file=./migrations/001_init_schema_d1.sql
```

Expected output:
```
🌀 Executing on remote database qhub-production (a52bc4fb-9185-4185-bb6e-572c3dd3feaf):
🌀 To execute on your local development database, remove the --remote flag from your wrangler command.
🚣 Executed 24 commands in 0.85s
```

## Step 4: Verify Tables Created

```bash
wrangler d1 execute qhub-production \
  --command="SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
```

Expected tables:
- api_keys
- oauth_connections
- quantum_jobs
- usage_records
- user_preferences
- user_sessions
- users

## Step 5: Create Cloudflare Worker

Create `wrangler.toml`:

```toml
name = "qhub-api"
main = "src/index.js"
compatibility_date = "2024-01-01"

[[d1_databases]]
binding = "DB"
database_name = "qhub-production"
database_id = "a52bc4fb-9185-4185-bb6e-572c3dd3feaf"

[vars]
ENVIRONMENT = "production"
API_BASE_URL = "https://api.qhub.dev"
```

## Step 6: Set Secrets in Cloudflare

**NEVER commit these!** Set them in Cloudflare dashboard or via CLI:

```bash
# Generate a secure JWT secret
openssl rand -base64 32

# Set secrets (replace with your actual values)
wrangler secret put JWT_SECRET
# Paste the generated secret when prompted

wrangler secret put CLOUDFLARE_AI_TOKEN
# Paste your Cloudflare AI token

wrangler secret put IBM_QUANTUM_TOKEN
# Paste your IBM Quantum token (optional)
```

## Step 7: Local Development with D1

For local testing against the remote D1 database:

```bash
# Create local .env file (DO NOT COMMIT)
cat > .env.production.local << EOF
DATABASE_URL=d1://a52bc4fb-9185-4185-bb6e-572c3dd3feaf
CLOUDFLARE_DATABASE_ID=a52bc4fb-9185-4185-bb6e-572c3dd3feaf
JWT_SECRET=$(openssl rand -base64 32)
CLOUDFLARE_AI_TOKEN=your-token-here
EOF

# Add to .gitignore (already done)
echo ".env.production.local" >> .gitignore
```

## Step 8: Query the Database

```bash
# List all users
wrangler d1 execute qhub-production \
  --command="SELECT email, tier, created_at FROM users"

# Check active sessions
wrangler d1 execute qhub-production \
  --command="SELECT COUNT(*) as active_sessions FROM user_sessions WHERE expires_at > strftime('%s', 'now')"

# View usage statistics
wrangler d1 execute qhub-production \
  --command="SELECT resource_type, COUNT(*) as count FROM usage_records GROUP BY resource_type"
```

## Step 9: Backup Strategy

```bash
# Export all data (run regularly)
wrangler d1 export qhub-production --output=backup-$(date +%Y%m%d).sql

# Store backups securely (e.g., S3, R2)
wrangler r2 object put qhub-backups/backup-$(date +%Y%m%d).sql \
  --file=backup-$(date +%Y%m%d).sql
```

## Step 10: Deploy to Production

```bash
# Build Rust backend (if needed)
cargo build --release

# Deploy Cloudflare Worker
wrangler deploy

# Verify deployment
curl https://api.qhub.dev/health
```

## Database Schema

The D1 database uses SQLite with these adaptations from PostgreSQL:

| PostgreSQL | D1 (SQLite) |
|------------|-------------|
| UUID | TEXT |
| TIMESTAMP WITH TIME ZONE | INTEGER (Unix timestamp) |
| JSONB | TEXT (JSON string) |
| INET | TEXT |
| BOOLEAN | INTEGER (0/1) |

## Monitoring

### View Logs
```bash
wrangler tail
```

### Check Database Size
```bash
wrangler d1 info qhub-production
```

### Usage Limits (Free Tier)
- 5 million reads/day
- 100,000 writes/day
- 5 GB storage

Monitor in [Cloudflare Dashboard](https://dash.cloudflare.com)

## Troubleshooting

### "Database not found"
```bash
# Verify database exists
wrangler d1 list

# Should show: qhub-production (a52bc4fb-9185-4185-bb6e-572c3dd3feaf)
```

### "Table already exists"
```bash
# Check what tables exist
wrangler d1 execute qhub-production \
  --command="SELECT name FROM sqlite_master WHERE type='table'"

# If migrations were already run, skip re-running
```

### Migration Failed
```bash
# Check syntax
sqlite3 test.db < migrations/001_init_schema_d1.sql

# If successful, try D1 again
wrangler d1 execute qhub-production \
  --file=./migrations/001_init_schema_d1.sql
```

## Security Checklist

- [ ] JWT_SECRET set in Cloudflare secrets (not in code)
- [ ] All API tokens stored as secrets
- [ ] wrangler.toml does NOT contain secrets
- [ ] Database ID in wrangler.toml is OK (not secret)
- [ ] Backups scheduled and tested
- [ ] Rate limiting enabled in Worker
- [ ] CORS configured properly
- [ ] Error messages don't leak sensitive data

## Production Deployment Workflow

1. **Test on staging** (PostgreSQL)
2. **Merge to main** (code review)
3. **Run migrations on D1**
4. **Update secrets in Cloudflare**
5. **Deploy with wrangler**
6. **Run smoke tests**
7. **Monitor logs and metrics**

## Useful Commands

```bash
# Open D1 console
wrangler d1 execute qhub-production

# Reset database (⚠️ DESTRUCTIVE)
# Drop all tables and re-run migrations
wrangler d1 execute qhub-production \
  --command="DROP TABLE IF EXISTS users"
# ... repeat for all tables, then run migrations

# Clone database to staging
wrangler d1 export qhub-production --output=prod-backup.sql
wrangler d1 execute qhub-staging --file=prod-backup.sql
```

## Support

- Cloudflare D1 Docs: https://developers.cloudflare.com/d1/
- Wrangler CLI: https://developers.cloudflare.com/workers/wrangler/
- Issues: https://github.com/CeoatNorthstar/qhub-cli/issues
