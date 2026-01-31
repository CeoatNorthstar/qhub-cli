# 🚀 Pre-Deployment Checklist

Use this checklist before running `./deploy.sh` to ensure a smooth deployment.

---

## ✅ Prerequisites

### Local Environment
- [ ] Node.js 18+ installed (`node --version`)
- [ ] npm installed (`npm --version`)
- [ ] Rust 1.70+ installed (`cargo --version`)
- [ ] Wrangler CLI working (`npx wrangler --version`)
- [ ] Cloudflare account created
- [ ] Logged into Cloudflare (`npx wrangler login`)

### Repository
- [ ] All changes committed to git
- [ ] Branch is up to date with main
- [ ] No uncommitted changes (`git status`)
- [ ] All tests passing locally (`./test_integration.sh`)

---

## 🗄️ Database Setup

### Staging Database
- [ ] Created staging database: `cd workers && npx wrangler d1 create qhub-staging`
- [ ] Copy database ID from output
- [ ] Update `workers/wrangler.toml` line 23 with staging database ID
- [ ] Applied migrations: `npx wrangler d1 migrations apply qhub-staging --env staging --remote`
- [ ] Verified migrations: `npx wrangler d1 migrations list qhub-staging --env staging --remote`

### Production Database
- [x] Production database already exists (ID: `a52bc4fb-9185-4185-bb6e-572c3dd3feaf`)
- [ ] Applied migrations: `npx wrangler d1 migrations apply qhub-production --env production --remote`
- [ ] Verified migrations: `npx wrangler d1 migrations list qhub-production --env production --remote`

---

## 🔐 Secrets Configuration

### Generate Strong JWT Secret
```bash
# Generate a strong 64-character secret
openssl rand -base64 48
# Or use: node -e "console.log(require('crypto').randomBytes(48).toString('base64'))"
```

### Set Staging Secrets
```bash
cd workers

# Set JWT secret (use the generated secret above)
npx wrangler secret put JWT_SECRET --env staging
# Paste your generated secret when prompted
```

### Set Production Secrets
```bash
cd workers

# Set JWT secret (use a DIFFERENT secret for production!)
npx wrangler secret put JWT_SECRET --env production
# Paste your generated production secret when prompted
```

### Verify Secrets
- [ ] JWT_SECRET set for staging
- [ ] JWT_SECRET set for production
- [ ] Secrets are different between environments
- [ ] Secrets are saved in your password manager

---

## 🧪 Pre-Deployment Testing

### Local Testing
```bash
# Run full integration test suite
./test_integration.sh

# Expected output:
# ✅ Backend is running
# ✅ Registration successful
# ✅ Token verification successful
# ✅ AI chat successful
# ✅ Logout successful
# ✅ CLI builds successfully
```

- [ ] All integration tests passing
- [ ] Backend starts without errors
- [ ] CLI connects to local backend
- [ ] Registration flow works
- [ ] Login flow works
- [ ] AI chat works
- [ ] Logout works

### Code Quality
- [ ] No TypeScript errors: `cd workers && npm run build`
- [ ] No Rust warnings: `cargo build --release 2>&1 | grep warning`
- [ ] Code formatted: `cargo fmt --check` (if applicable)
- [ ] No TODOs in critical paths: `grep -r "TODO" src/ workers/src/`

---

## 📝 Configuration Review

### wrangler.toml
- [ ] `name = "qhub-api"` is correct
- [ ] Staging environment configured
- [ ] Production environment configured
- [ ] Production database ID is correct (a52bc4fb-9185-4185-bb6e-572c3dd3feaf)
- [ ] Compatibility date is recent
- [ ] AI binding configured
- [ ] Environment variables set (ENVIRONMENT, JWT_EXPIRY_HOURS)

### Environment Variables
- [ ] `.dev.vars` has JWT_SECRET for local development
- [ ] `.dev.vars` is in `.gitignore` (never commit secrets!)
- [ ] Production secrets stored securely (Cloudflare, not in git)

---

## 🌐 Cloudflare Dashboard

### Verify Access
- [ ] Logged into Cloudflare dashboard: https://dash.cloudflare.com
- [ ] Can see D1 databases in sidebar
- [ ] Can see Workers & Pages in sidebar
- [ ] Have necessary permissions (Admin or Workers role)

### Check Quotas
- [ ] Workers free tier: 100,000 requests/day (or paid plan)
- [ ] D1 free tier: 5GB storage, 5M reads/day (or paid plan)
- [ ] AI free tier: Check limits in dashboard

---

## 📦 Deployment Readiness

### Staging Deployment
- [ ] Staging database created and migrated
- [ ] Staging secrets configured
- [ ] Local tests passing
- [ ] Ready to deploy to staging

### Production Deployment
- [ ] Staging deployed and tested
- [ ] Production database migrated
- [ ] Production secrets configured
- [ ] Backup plan in place (can rollback)
- [ ] Monitoring ready (Cloudflare dashboard)
- [ ] Ready to deploy to production

---

## 🚀 Deployment Steps

Once all items above are checked, run:

```bash
./deploy.sh
```

The script will guide you through:
1. Choose environment (staging/production/both)
2. Verify prerequisites
3. Check database and secrets
4. Apply migrations
5. Run tests
6. Deploy to Cloudflare Workers
7. Test deployed endpoints
8. Build release CLI binary
9. Optional: Install CLI system-wide

---

## 🧪 Post-Deployment Testing

### Staging
```bash
# Test health endpoint
curl https://qhub-api-staging.workers.dev/health

# Test registration
curl -X POST https://qhub-api-staging.workers.dev/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"Test123!","username":"test"}'

# Configure CLI for staging
export QHUB_API_URL=https://qhub-api-staging.workers.dev
cargo run
# Then: /register email username password
```

- [ ] Health endpoint returns 200
- [ ] Registration works
- [ ] Login works
- [ ] AI chat works
- [ ] CLI connects successfully

### Production
```bash
# Test health endpoint
curl https://qhub-api-production.workers.dev/health

# Configure CLI for production
export QHUB_API_URL=https://qhub-api-production.workers.dev
cargo run
# Then: /register your-email username secure-password
```

- [ ] Health endpoint returns 200
- [ ] Registration works
- [ ] Login works
- [ ] AI chat works
- [ ] CLI connects successfully
- [ ] Performance is acceptable (<100ms response times)

---

## 📊 Monitoring

### Cloudflare Dashboard
- [ ] Workers → qhub-api-staging → Analytics
- [ ] Workers → qhub-api-production → Analytics
- [ ] Check logs for errors
- [ ] Check request rates
- [ ] Check error rates

### Set Up Alerts (Optional)
- [ ] Configure error rate alerts
- [ ] Configure usage alerts (approaching quotas)
- [ ] Configure uptime monitoring (e.g., UptimeRobot)

---

## 🔄 Rollback Plan

If something goes wrong:

### Quick Rollback
```bash
# Rollback Workers deployment
cd workers
npx wrangler rollback --env production

# Or redeploy previous version
git checkout <previous-commit>
npx wrangler deploy --env production
```

### Database Rollback
- [ ] Have database backup plan
- [ ] Know how to restore from backup
- [ ] Test rollback procedure in staging first

---

## 📚 Documentation

### Update Documentation
- [ ] Update README.md with production URLs
- [ ] Update API documentation with live endpoints
- [ ] Document any configuration changes
- [ ] Update troubleshooting guide if needed

### Share Access
- [ ] Share production URL with team
- [ ] Share API documentation
- [ ] Share deployment credentials (securely!)
- [ ] Set up team access in Cloudflare dashboard

---

## 🎯 Success Criteria

Deployment is successful when:
- [x] Backend deployed without errors
- [x] Health endpoint responds
- [x] All API endpoints working
- [x] Authentication flow works
- [x] AI chat works
- [x] CLI connects and works
- [x] No errors in Cloudflare logs
- [x] Response times <100ms
- [x] Tests passing against live endpoints

---

## 🚨 Emergency Contacts

**If deployment fails:**
1. Check Cloudflare dashboard logs
2. Run `./test_integration.sh` locally
3. Review this checklist for missed steps
4. Check #deployment channel (if you have one)
5. Contact: [Your team contact info]

---

## ✅ Final Check

Before clicking deploy:
- [ ] Read this entire checklist
- [ ] All items above are checked
- [ ] Secrets are configured
- [ ] Migrations are applied
- [ ] Tests are passing
- [ ] Backup plan in place
- [ ] Team is informed
- [ ] Coffee/tea prepared ☕

**Ready to deploy?** Run: `./deploy.sh`

---

**Good luck! 🚀**
