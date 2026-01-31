# QHub Database Setup

This guide explains how to set up the database for local development and production deployment.

## Architecture

QHub supports two database backends:
- **PostgreSQL** (local development, staging)
- **Cloudflare D1** (production, serverless SQLite)

## Local Development Setup (PostgreSQL)

### 1. Start PostgreSQL with Docker

```bash
# Start PostgreSQL container
docker-compose up -d

# Verify it's running
docker ps | grep qhub-postgres
```

### 2. Create .env.local file

```bash
cp .env.example .env.local
```

Edit `.env.local`:
```env
DATABASE_URL=postgresql://qhub:qhub_dev_password@localhost:5432/qhub
ENVIRONMENT=development
JWT_SECRET=your-development-secret-key
```

### 3. Run migrations

Migrations run automatically when the application starts, or you can run them manually:

```bash
# Install sqlx-cli
cargo install sqlx-cli --features postgres

# Run migrations
sqlx migrate run
```

### 4. Test the connection

```bash
# Load environment
export $(cat .env.local | xargs)

# Run the application
cargo run
```

## Staging Branch Setup

The `staging` branch is for testing with a local PostgreSQL database before deploying to production.

### Create and switch to staging branch

```bash
# Create staging branch from main
git checkout -b staging

# Copy local environment
cp .env.local .env

# Start development
docker-compose up -d
cargo run
```

### Staging workflow

```bash
# Work on features in feature branches
git checkout -b feature/my-feature

# Test locally
cargo test
cargo run

# Merge to staging for integration testing
git checkout staging
git merge feature/my-feature

# Test in staging environment
docker-compose up -d
cargo run

# After testing, merge to main
git checkout main
git merge staging
```

## Production Setup (Cloudflare D1)

### 1. Install Wrangler CLI

```bash
npm install -g wrangler
wrangler login
```

### 2. Create D1 Database

```bash
# Create production database
wrangler d1 create qhub-production

# Note the database ID from output
```

### 3. Configure .env.production

```bash
# Edit .env.production with your Cloudflare credentials
CLOUDFLARE_ACCOUNT_ID=your-account-id
CLOUDFLARE_DATABASE_ID=your-database-id
CLOUDFLARE_API_TOKEN=your-api-token
```

### 4. Run migrations on D1

```bash
# Apply migrations to Cloudflare D1
wrangler d1 execute qhub-production --file=./migrations/001_init_schema_d1.sql
```

### 5. Deploy to production

```bash
# Use production environment
export $(cat .env.production | xargs)

# Build release
cargo build --release

# Deploy (deployment method depends on your setup)
```

## Database Schema

See `migrations/001_init_schema.sql` for the complete schema.

### Main Tables

- **users** - User accounts
- **user_sessions** - Active login sessions
- **oauth_connections** - OAuth provider linkages
- **api_keys** - API keys for programmatic access
- **user_preferences** - User settings
- **usage_records** - Usage tracking for quotas
- **quantum_jobs** - Quantum computation history

## Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `DATABASE_URL` | Database connection string | Yes |
| `JWT_SECRET` | Secret key for JWT tokens | Yes |
| `TOKEN_EXPIRY_HOURS` | Token expiration (default: 24) | No |
| `ENVIRONMENT` | Environment name | No |

## Branch Strategy

### main (production)
- Uses Cloudflare D1
- Production-ready code only
- Protected branch
- Requires PR reviews

### staging (testing)
- Uses local PostgreSQL
- Integration testing
- Merge here before main
- Tests with production-like data

### feature/* (development)
- Uses local PostgreSQL
- Individual feature development
- Merge to staging when ready

## Database Migrations

### Creating a new migration

```bash
# Create migration file
sqlx migrate add <migration_name>

# Edit the generated file in migrations/
vim migrations/<timestamp>_<migration_name>.sql

# Test locally
sqlx migrate run

# For D1, also create a D1-compatible version
cp migrations/<timestamp>_<migration_name>.sql \
   migrations/<timestamp>_<migration_name>_d1.sql

# Adjust for D1 (SQLite) compatibility
```

### Migration best practices

1. **Always test locally first** with PostgreSQL
2. **Create D1 version** for production compatibility
3. **Make migrations reversible** when possible
4. **Test rollback** procedures
5. **Document breaking changes**

## Troubleshooting

### Connection refused (PostgreSQL)

```bash
# Check if container is running
docker ps | grep qhub-postgres

# Restart container
docker-compose restart postgres

# Check logs
docker-compose logs postgres
```

### Migration failed

```bash
# Check current migration version
sqlx migrate info

# Revert last migration
sqlx migrate revert

# Run migrations again
sqlx migrate run
```

### D1 quota limits

Cloudflare D1 has the following limits:
- Free tier: 5 million reads/day, 100k writes/day
- Paid tier: Higher limits available

Monitor usage in Cloudflare dashboard.

## Security Considerations

1. **Never commit `.env.local`** to version control
2. **Rotate JWT_SECRET** in production regularly
3. **Use strong passwords** for PostgreSQL
4. **Enable SSL** for production database connections
5. **Backup D1 database** regularly
6. **Limit database user permissions** to minimum required

## Backup and Restore

### PostgreSQL (local/staging)

```bash
# Backup
docker exec qhub-postgres pg_dump -U qhub qhub > backup.sql

# Restore
docker exec -i qhub-postgres psql -U qhub qhub < backup.sql
```

### Cloudflare D1 (production)

```bash
# Export data
wrangler d1 execute qhub-production --command="SELECT * FROM users" --json > users_backup.json

# Import data
# (Use wrangler d1 execute with INSERT statements)
```

## Performance Tips

1. **Use connection pooling** (already configured in code)
2. **Add indexes** for frequently queried columns
3. **Monitor slow queries** with `EXPLAIN ANALYZE`
4. **Use prepared statements** (sqlx does this automatically)
5. **Implement caching** for frequently accessed data

## Support

For database issues:
1. Check Docker logs: `docker-compose logs`
2. Verify environment variables: `printenv | grep DATABASE`
3. Test connection: `psql postgresql://qhub:qhub_dev_password@localhost:5432/qhub`
