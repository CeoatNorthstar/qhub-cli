# QHub Workers - Deployment Checklist

Use this checklist before deploying to staging or production.

## Pre-Deployment

### Code Quality
- [ ] All TypeScript compilation passes (`npm run tsc --noEmit`)
- [ ] Code follows project style guidelines
- [ ] No sensitive data (passwords, tokens) in code
- [ ] All TODOs addressed or documented
- [ ] Code reviewed by team member

### Testing
- [ ] Manual testing completed locally (`npm run dev`)
- [ ] All endpoints tested with example requests
- [ ] Authentication flow tested
- [ ] Error handling verified
- [ ] Edge cases tested
- [ ] Usage limits verified

### Database
- [ ] Migration files created
- [ ] Migrations tested locally
- [ ] Indexes added for new queries
- [ ] Foreign keys properly configured
- [ ] Database backup created (for production)

### Documentation
- [ ] README.md updated with new features
- [ ] API_EXAMPLES.md updated with new endpoints
- [ ] DEVELOPMENT.md updated if architecture changed
- [ ] Inline code comments added for complex logic
- [ ] CHANGELOG updated (if exists)

## Staging Deployment

### Preparation
- [ ] Staging database created (`wrangler d1 create qhub-staging`)
- [ ] Database ID updated in wrangler.toml
- [ ] Migrations run on staging (`npm run db:migrate:staging`)
- [ ] Secrets set in staging environment
  - [ ] `JWT_SECRET`
  - [ ] Other secrets as needed

### Deploy
- [ ] Deploy to staging: `npm run deploy:staging`
- [ ] Deployment successful (no errors)
- [ ] Worker URL noted: `https://qhub-api-staging.workers.dev`

### Verification
- [ ] Health check passes: `curl https://qhub-api-staging.workers.dev/health`
- [ ] Register new user works
- [ ] Login works
- [ ] Protected endpoints work with token
- [ ] AI chat endpoint works
- [ ] Quantum job submission works
- [ ] Error responses correct (401, 404, etc.)
- [ ] Check logs: `wrangler tail --env staging`

### Monitoring
- [ ] Cloudflare dashboard shows requests
- [ ] No error spike in logs
- [ ] Response times acceptable
- [ ] Database queries executing correctly

## Production Deployment

### Final Checks
- [ ] Staging environment fully tested
- [ ] All checklist items above completed
- [ ] Team notified of deployment
- [ ] Deployment window scheduled (if applicable)
- [ ] Rollback plan prepared

### Preparation
- [ ] Production database created (if new)
- [ ] Database backup completed
- [ ] Migrations ready for production
- [ ] Secrets verified in production environment
  - [ ] `JWT_SECRET` (strong, unique)
  - [ ] Other production secrets
- [ ] Environment variables set correctly in wrangler.toml

### Deploy
- [ ] Run migrations: `npm run db:migrate:production`
- [ ] Deploy to production: `npm run deploy:production`
- [ ] Deployment successful
- [ ] Production URL noted

### Post-Deployment Verification
- [ ] Health check: `curl https://qhub-api.workers.dev/health`
- [ ] API info endpoint works
- [ ] Create test account (use test email)
- [ ] Test login
- [ ] Test protected endpoints
- [ ] Test AI chat with test account
- [ ] Test quantum job submission
- [ ] Verify usage limits enforced
- [ ] Check error handling

### Monitoring (First Hour)
- [ ] Monitor logs: `wrangler tail --env production`
- [ ] Check Cloudflare dashboard
  - [ ] Request volume normal
  - [ ] Error rate acceptable (<1%)
  - [ ] Response times good (<100ms)
  - [ ] No 5xx errors
- [ ] Database performance acceptable
- [ ] No user-reported issues

### Monitoring (First 24 Hours)
- [ ] Daily check of error logs
- [ ] Monitor usage patterns
- [ ] Check for performance degradation
- [ ] Verify database size growth is expected
- [ ] Check for any security alerts

## Rollback Procedure

If issues are detected:

1. **Immediate Rollback**
   ```bash
   wrangler deployments list --env production
   wrangler rollback [deployment-id] --env production
   ```

2. **Verify Rollback**
   - [ ] Previous version deployed
   - [ ] Health check passes
   - [ ] Critical functionality works

3. **Investigate**
   - [ ] Check logs for errors
   - [ ] Review recent changes
   - [ ] Test fix locally
   - [ ] Re-deploy when ready

4. **Communicate**
   - [ ] Notify team of rollback
   - [ ] Document issue
   - [ ] Update incident log

## Post-Deployment

### Documentation
- [ ] Deployment logged in change log
- [ ] Any issues documented
- [ ] Lessons learned noted

### Cleanup
- [ ] Test accounts removed (if created)
- [ ] Temporary data cleaned up
- [ ] Local environment synced with production

### Communication
- [ ] Team notified of successful deployment
- [ ] Users informed of new features (if applicable)
- [ ] Documentation updated with new endpoints

## Environment-Specific Notes

### Staging
- Used for: Testing, QA, pre-production verification
- Data: Test data only, can be reset
- Access: Internal team only
- Monitoring: Casual, check before production deploy

### Production
- Used for: Live user traffic
- Data: Real user data, must be protected
- Access: Public API, authenticated users
- Monitoring: Continuous, alerting enabled

## Security Checklist

- [ ] JWT_SECRET is strong and unique per environment
- [ ] No secrets in code or logs
- [ ] Database queries use parameterization
- [ ] Input validation on all endpoints
- [ ] Rate limiting considered
- [ ] CORS configured appropriately
- [ ] HTTPS enforced (Cloudflare handles this)
- [ ] User data encrypted at rest (D1 handles this)

## Performance Checklist

- [ ] Database queries optimized
- [ ] Indexes in place for common queries
- [ ] Response payloads kept small
- [ ] Pagination used for large datasets
- [ ] No N+1 query problems
- [ ] Cache headers set appropriately

## Compliance Checklist

- [ ] User data handling compliant with policies
- [ ] Privacy policy updated (if user-facing)
- [ ] Terms of service updated (if applicable)
- [ ] Data retention policies followed
- [ ] Logging doesn't include PII

## Sign-Off

### Deployer
- Name: _______________
- Date: _______________
- Time: _______________

### Reviewer (if required)
- Name: _______________
- Date: _______________
- Approved: [ ] Yes [ ] No

### Notes
_Use this space for any deployment notes, issues encountered, or special considerations:_

---

**Remember**: When in doubt, test more. It's better to delay a deployment than to rush and cause issues.
