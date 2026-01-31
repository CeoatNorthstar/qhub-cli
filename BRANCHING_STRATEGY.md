# Git Branching Strategy - QHub Enterprise Workflow

**Version**: 1.0  
**Last Updated**: 2026-01-31  
**Status**: ✅ Implemented & Active

---

## 🎯 Overview

QHub uses a **two-branch strategy** with automatic configuration protection:

- **`staging` branch** → Staging environment (test before production)
- **`main` branch** → Production environment (live users)

**Key Feature**: When merging `staging` → `main`, code changes merge but **production configs stay protected**.

---

## 🏗️ Branch Structure

```
main (production)
  ├─ Database: b607a2f3-0ab5-407e-b63a-eb21d01084d0
  ├─ API: qhub-api-production.a-contactnaol.workers.dev
  └─ Protected: Cannot push directly

staging (pre-production)
  ├─ Database: 18a877b3-4dd2-4f07-bf92-2b341bf8a2ba
  ├─ API: qhub-api-staging.a-contactnaol.workers.dev
  └─ Active development branch
```

---

## 🔒 Automatic Config Protection

### How It Works

1. **Git Attributes** (`.gitattributes`)
   - Marks config files with special merge strategies
   - Ensures `main` keeps production configs during merges
   - No manual conflict resolution needed

2. **Branch Validation** (`deploy.sh`)
   - Refuses to deploy production from `staging` branch
   - Warns when deploying staging from `main` branch
   - Validates database IDs before deployment

3. **Smart CLI Defaults** (`src/config/settings.rs`)
   - Debug builds → `localhost:8787`
   - Release builds → Staging URL
   - Production feature → Production URL

---

## 📋 Developer Workflow

### Working on Features

```bash
# 1. Switch to staging branch
git checkout staging
git pull origin staging

# 2. Create feature branch (optional)
git checkout -b feature/my-feature

# 3. Make your changes
# ... edit code ...

# 4. Commit changes
git add .
git commit -m "Add new feature"

# 5. Push to staging
git checkout staging  # if on feature branch
git merge feature/my-feature  # if on feature branch
git push origin staging

# ✅ Staging automatically deploys (if CI/CD configured)
```

### Testing in Staging

```bash
# Test the staging deployment
curl https://qhub-api-staging.a-contactnaol.workers.dev/health

# Test with CLI
export QHUB_API_URL=https://qhub-api-staging.a-contactnaol.workers.dev
cargo run
# /register test@staging.com testuser TestPass123!
```

### Deploying to Production

```bash
# 1. Switch to main branch
git checkout main
git pull origin main

# 2. Merge staging to main
git merge staging

# 3. Review the merge
git log --oneline -5
git diff HEAD~1

# 4. Push to main
git push origin main

# ✅ Production automatically deploys (if CI/CD configured)
# OR manually: ./deploy.sh (select option 2)
```

---

## 🛡️ Safety Guarantees

### What's Protected During Merge

✅ **Environment files** (`.env`, `.env.production`, `.env.staging`)  
✅ **Worker secrets** (managed via Cloudflare, not in git)  
✅ **Database IDs** (validated in `deploy.sh`)  
✅ **API URLs** (protected in config files)

### What Merges Normally

✅ **Source code** (`src/`, `workers/src/`)  
✅ **Documentation** (`.md` files)  
✅ **Tests** (`test_*.sh`)  
✅ **Dependencies** (`Cargo.toml`, `package.json`)

---

## ⚠️ Important Rules

### DO ✅

- Always develop on `staging` branch
- Test thoroughly in staging before merging to `main`
- Run `./setup-env.sh` to configure environment for current branch
- Use `./deploy.sh` for deployments (validates branch)
- Review merge commits before pushing

### DON'T ❌

- Never push directly to `main` (merge through PR if branch protection enabled)
- Never manually edit database IDs in `wrangler.toml` without team agreement
- Never commit real secrets (JWT_SECRET) to any branch
- Never bypass the deployment script
- Never force-push to `main` or `staging`

---

## 🔧 Configuration Files

### Files That Stay Environment-Specific

| File | Staging Branch | Main Branch |
|------|---------------|-------------|
| `.env.staging` | ✅ Committed | ❌ Ignored |
| `.env.production` | ❌ Ignored | ✅ Committed |
| `.env` | Generated | Generated |

### Files That Merge Normally

| File | Purpose | Merge Strategy |
|------|---------|----------------|
| `wrangler.toml` | Has both envs | Union (keep both sections) |
| `src/**/*.rs` | Source code | Normal merge |
| `workers/src/**/*.ts` | Backend code | Normal merge |
| `*.md` | Documentation | Union merge |

---

## 🧪 Testing the Protection

### Test 1: Config Protection

```bash
# On staging branch
echo "TEST_STAGING=true" >> .env.staging
git add .env.staging
git commit -m "Test staging config"

# Merge to main
git checkout main
git merge staging

# Check that .env.production is unchanged
cat .env.production  # Should NOT have TEST_STAGING
```

### Test 2: Deployment Protection

```bash
# Try to deploy production from staging (should fail)
git checkout staging
./deploy.sh  # Choose option 2 (Production)

# Expected output:
# ❌ Cannot deploy PRODUCTION from 'staging' branch
```

### Test 3: Database ID Validation

```bash
# Temporarily break database ID
sed -i 's/b607a2f3/WRONG-ID/' workers/wrangler.toml

# Try to deploy
./deploy.sh  # Choose option 2 (Production)

# Expected output:
# ❌ Database ID mismatch!
```

---

## 🚨 Troubleshooting

### Problem: Merge Conflicts on Config Files

**Solution**: This shouldn't happen with git attributes, but if it does:

```bash
# Accept main branch version
git checkout --ours .env.production
git add .env.production

# Or accept staging branch version (if merging other direction)
git checkout --theirs .env.staging
git add .env.staging

git commit
```

### Problem: Wrong API URL in CLI

**Solution**: Run environment setup script:

```bash
./setup-env.sh
# This copies branch-specific .env to .env
```

### Problem: Deployed to Wrong Environment

**Solution**: Rollback immediately:

```bash
# If just deployed (not pushed)
git reset --hard HEAD~1

# If already pushed
git revert <merge-commit-hash>
git push origin main

# Then redeploy correct environment
./deploy.sh
```

---

## 📊 Deployment Checklist

Before deploying to production:

- [ ] All staging tests passing
- [ ] Feature tested in staging environment
- [ ] Database migrations applied to staging
- [ ] No errors in staging logs (Cloudflare dashboard)
- [ ] Team notified of pending deployment
- [ ] On `main` branch (`git branch --show-current`)
- [ ] Staging merged to main (`git merge staging`)
- [ ] No conflicts in config files
- [ ] Database IDs validated (`grep database_id workers/wrangler.toml`)
- [ ] Ready to deploy (`./deploy.sh`)

After deployment:

- [ ] Production health check passes
- [ ] Test basic functionality (register, login, AI chat)
- [ ] Monitor Cloudflare dashboard for errors
- [ ] Verify database queries are working
- [ ] Alert team that deployment is complete

---

## 🔄 Rollback Procedure

If production deployment fails:

1. **Immediate Rollback** (if not pushed):
   ```bash
   git reset --hard HEAD~1
   ```

2. **Revert Merge** (if pushed):
   ```bash
   git revert -m 1 <merge-commit>
   git push origin main
   ```

3. **Redeploy Previous Version**:
   ```bash
   git checkout <previous-working-commit>
   ./deploy.sh  # Deploy to production
   ```

4. **Investigate and Fix**:
   - Check Cloudflare logs
   - Review merge diff
   - Fix issues on staging
   - Test thoroughly
   - Merge to main again

---

## 📚 Additional Resources

- **Deployment Guide**: `DEPLOYMENT_CHECKLIST.md`
- **API Documentation**: `workers/README.md`
- **Integration Testing**: `test_integration.sh`
- **Architecture**: `ARCHITECTURE_COMPLETE.md`
- **Database Isolation**: `DATABASE_ISOLATION_VERIFIED.md`

---

## 🎯 Success Criteria

Your branching strategy is working correctly if:

- [x] Can merge staging to main without manual config edits
- [x] Main branch always points to production database
- [x] Staging branch always points to staging database
- [x] Cannot accidentally deploy wrong environment
- [x] Deployment script validates branch before deploying
- [x] CLI automatically uses correct API URL per environment
- [x] Team understands the workflow

---

## 💡 Tips & Best Practices

1. **Always run `./setup-env.sh` after switching branches**
   - Ensures your `.env` matches the current branch

2. **Use feature branches for big changes**
   - Branch from `staging`, merge back to `staging`
   - Keeps staging branch clean

3. **Test merges locally first**
   - Before pushing, test that everything works

4. **Review merge commits carefully**
   - Check that no production configs were changed accidentally

5. **Keep staging and main in sync**
   - Merge staging to main regularly (at least weekly)
   - Don't let them diverge too much

---

**Questions?** Review this document or check the team wiki.

**Changes to this workflow?** Discuss with the team first, update this doc.

---

**Maintained by**: DevOps Team  
**Last Review**: 2026-01-31  
**Next Review**: 2026-02-28
