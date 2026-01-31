# Security Policy

## Reporting Security Issues

If you discover a security vulnerability, please **DO NOT** create a public GitHub issue.

Instead, please email: security@qhub.dev (or create a private security advisory)

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We take security seriously and will respond within 48 hours.

## Security Best Practices

### Environment Variables

**❌ NEVER commit these files:**
- `.env`
- `.env.local`
- `.env.production`
- `.env.staging`
- Any file with real credentials

**✅ Safe to commit:**
- `.env.example` (template with placeholder values only)

### Checking for Leaked Secrets

Before committing, verify no secrets are staged:

```bash
# Check what's staged
git diff --cached

# Check for env files
git ls-files | grep -E "\.env"

# Should only show: .env.example
```

### If You Accidentally Committed Secrets

1. **Immediately rotate all exposed credentials**
2. Remove from Git history:
   ```bash
   git filter-repo --path .env.production --invert-paths
   # Or use BFG Repo-Cleaner
   ```
3. Force push (⚠️ breaks history for collaborators):
   ```bash
   git push origin --force --all
   ```
4. Notify the team

### JWT Secret Generation

Never use default secrets in production:

```bash
# Generate a secure JWT secret
openssl rand -base64 32

# Set it in your environment
export JWT_SECRET="<generated-secret>"
```

### Password Security

- Passwords are hashed with **Argon2** (OWASP recommended)
- Minimum 8 characters (enforced at application level)
- Hashes are never logged or exposed in API responses

### Database Security

#### PostgreSQL (Local/Staging)
- Use strong passwords (not "devpass" in production!)
- Restrict network access (localhost only for dev)
- Enable SSL for remote connections
- Regular backups

#### Cloudflare D1 (Production)
- Access controlled via Cloudflare API tokens
- Tokens scoped to minimum required permissions
- Rotate tokens regularly (every 90 days)
- Monitor usage in Cloudflare dashboard

### API Keys

- Store in environment variables, never in code
- Use different keys for dev/staging/production
- Implement rate limiting (done in code)
- Monitor for unusual usage patterns
- Rotate keys every 90 days

### Session Management

- Sessions expire after 24 hours (configurable)
- Tokens are hashed before storage (SHA-256)
- Old sessions cleaned up automatically
- Logout invalidates sessions immediately

## Dependency Security

### Regular Updates

```bash
# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Audit for known vulnerabilities
cargo audit
```

### Supply Chain Security

- All dependencies pinned in `Cargo.lock`
- Use `cargo-deny` to check licenses and vulnerabilities
- Review dependency changes before updating

## Infrastructure Security

### Cloudflare Workers (Production)

- Environment variables set in Cloudflare dashboard
- No secrets in wrangler.toml
- Use Secrets (not environment variables) for sensitive data
- Enable rate limiting and DDoS protection

### Docker (Local Development)

- Don't expose PostgreSQL port to 0.0.0.0 in production
- Use Docker secrets for credentials
- Regular security updates: `docker pull postgres:16-alpine`

## Authentication Flow Security

1. **Password Reset**
   - Tokens expire after 1 hour
   - One-time use only
   - Sent via secure email (HTTPS)

2. **OAuth**
   - State parameter prevents CSRF
   - Tokens encrypted at rest
   - Refresh tokens rotated

3. **API Keys**
   - Long, random, unpredictable
   - Scoped permissions
   - Can be revoked instantly

## Compliance

### GDPR

- User data can be deleted on request
- Data export available
- Privacy policy required before launch

### Data Retention

- User sessions: 24 hours
- Quantum jobs: 90 days (configurable)
- Audit logs: 1 year

## Security Checklist for Production

- [ ] All secrets in environment variables (not code)
- [ ] JWT_SECRET is a strong random value
- [ ] Database uses SSL/TLS
- [ ] API keys are different from dev/staging
- [ ] Rate limiting enabled
- [ ] HTTPS enforced (Cloudflare handles this)
- [ ] Error messages don't leak sensitive info
- [ ] Logging doesn't include PII or secrets
- [ ] Dependencies are up to date
- [ ] Security headers configured
- [ ] CORS properly configured
- [ ] Input validation on all endpoints

## Contact

Security Team: security@qhub.dev
General Issues: https://github.com/CeoatNorthstar/qhub-cli/issues
