# 🎯 Quick Deployment Reference

**TL;DR for experienced devs who know what they're doing.**

---

## ⚡ Fast Track Deployment

### 1. Prerequisites Check (30 seconds)
```bash
node --version  # Need 18+
cargo --version # Need 1.70+
npx wrangler login  # Login to Cloudflare
```

### 2. Database Setup (2 minutes)

**Create staging database:**
```bash
cd workers
npx wrangler d1 create qhub-staging
# Copy database_id from output, update wrangler.toml line 23
```

**Apply migrations:**
```bash
# Staging
npx wrangler d1 migrations apply qhub-staging --env staging --remote

# Production (already exists: a52bc4fb-9185-4185-bb6e-572c3dd3feaf)
npx wrangler d1 migrations apply qhub-production --env production --remote
```

### 3. Set Secrets (1 minute)

**Generate secret:**
```bash
openssl rand -base64 48
```

**Set secrets:**
```bash
cd workers
npx wrangler secret put JWT_SECRET --env staging
npx wrangler secret put JWT_SECRET --env production
```

### 4. Deploy (1 minute)

**Automated:**
```bash
cd ..
./deploy.sh
# Choose option 3 (both staging and production)
```

**Manual:**
```bash
cd workers

# Staging
npx wrangler deploy --env staging

# Production
npx wrangler deploy --env production
```

### 5. Test (30 seconds)
```bash
curl https://qhub-api-staging.workers.dev/health
curl https://qhub-api-production.workers.dev/health
```

### 6. Build CLI (30 seconds)
```bash
cargo build --release
sudo cp target/release/qhub /usr/local/bin/
```

---

## 🔧 Essential Commands

### Database
```bash
# List databases
npx wrangler d1 list

# List migrations
npx wrangler d1 migrations list <db-name> --env <env> --remote

# Execute SQL
npx wrangler d1 execute <db-name> --env <env> --remote --command "SELECT COUNT(*) FROM users"
```

### Secrets
```bash
# Set secret
npx wrangler secret put <NAME> --env <env>

# List secrets (names only, not values)
npx wrangler secret list --env <env>

# Delete secret
npx wrangler secret delete <NAME> --env <env>
```

### Deployment
```bash
# Deploy
npx wrangler deploy --env <env>

# Rollback (if something breaks)
npx wrangler rollback --env <env>

# View logs
npx wrangler tail --env <env>
```

### Testing
```bash
# Local integration tests
./test_integration.sh

# Test live endpoint
curl https://qhub-api-<env>.workers.dev/health

# Test with CLI
export QHUB_API_URL=https://qhub-api-<env>.workers.dev
cargo run
```

---

## 🌐 URLs

### Staging
- **API**: https://qhub-api-staging.workers.dev
- **Dashboard**: https://dash.cloudflare.com → Workers → qhub-api-staging

### Production
- **API**: https://qhub-api-production.workers.dev
- **Dashboard**: https://dash.cloudflare.com → Workers → qhub-api-production

---

## 📊 Database IDs

- **Local**: `local` (in .wrangler/state)
- **Staging**: [Create with `wrangler d1 create qhub-staging`]
- **Production**: `a52bc4fb-9185-4185-bb6e-572c3dd3feaf`

---

## 🚨 Troubleshooting

### Deployment fails
```bash
# Check logs
npx wrangler tail --env <env>

# Verify migrations
npx wrangler d1 migrations list <db-name> --env <env> --remote

# Check secrets
npx wrangler secret list --env <env>
```

### API returns errors
```bash
# Check logs in Cloudflare dashboard
# Or: npx wrangler tail --env <env>

# Test locally first
cd workers && npm run dev
curl http://localhost:8787/health
```

### CLI can't connect
```bash
# Verify API URL
echo $QHUB_API_URL
cat ~/.qhub/config.toml

# Test API directly
curl $QHUB_API_URL/health

# Clear local config
rm ~/.qhub/config.toml
```

---

## 📈 Monitoring

### Quick Health Check
```bash
# All endpoints should return 200
curl -w "\n%{http_code}\n" https://qhub-api-staging.workers.dev/health
curl -w "\n%{http_code}\n" https://qhub-api-production.workers.dev/health
```

### View Metrics
- Dashboard: https://dash.cloudflare.com
- Workers → Select worker → Analytics tab
- Check: Request rate, Error rate, CPU time

### View Logs
```bash
# Real-time logs
npx wrangler tail --env production

# Or in dashboard:
# Workers → Select worker → Logs (Real-time Logs)
```

---

## 🔄 Common Operations

### Update Code
```bash
# 1. Make changes
# 2. Test locally
npm run dev  # in workers/
./test_integration.sh

# 3. Deploy
npx wrangler deploy --env staging  # test first
npx wrangler deploy --env production  # then prod
```

### Add Migration
```bash
# 1. Create migration file
cd workers/migrations
# Create 002_your_migration.sql

# 2. Test locally
npx wrangler d1 migrations apply qhub-dev --local

# 3. Apply to staging
npx wrangler d1 migrations apply qhub-staging --env staging --remote

# 4. Apply to production
npx wrangler d1 migrations apply qhub-production --env production --remote
```

### Rollback
```bash
# Workers deployment
npx wrangler rollback --env production

# Database (no built-in rollback - need backup strategy)
# Best practice: Test in staging first!
```

---

## 🎯 One-Liner for Each Task

```bash
# Quick deploy to staging
cd workers && npx wrangler deploy --env staging

# Quick deploy to production
cd workers && npx wrangler deploy --env production

# View staging logs
cd workers && npx wrangler tail --env staging

# Test staging
curl https://qhub-api-staging.workers.dev/health

# Test production
curl https://qhub-api-production.workers.dev/health

# Build release CLI
cargo build --release

# Install CLI
sudo cp target/release/qhub /usr/local/bin/

# Run local backend
cd workers && npm run dev

# Run integration tests
./test_integration.sh
```

---

## 📚 Full Documentation

For detailed information, see:
- **DEPLOYMENT_CHECKLIST.md** - Complete pre-deployment checklist
- **INTEGRATION_GUIDE.md** - Setup and integration guide
- **workers/README.md** - Backend API documentation
- **workers/DEPLOYMENT_CHECKLIST.md** - Backend-specific deployment

---

**That's it! Deploy with confidence.** 🚀
