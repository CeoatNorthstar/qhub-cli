# QHub API - Quick Reference Card

## 🚀 Quick Start

```bash
npm install                    # Install dependencies
./setup.sh                     # Run setup wizard
npm run dev                    # Start dev server (http://localhost:8787)
```

## 📡 Base URLs

- **Local**: `http://localhost:8787`
- **Staging**: `https://qhub-api-staging.workers.dev`
- **Production**: `https://qhub-api.workers.dev`

## 🔑 Authentication

```bash
# Set your token after login/register
export TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

# Use in requests
curl -H "Authorization: Bearer $TOKEN" ...
```

## 📋 Common Commands

### Development
```bash
npm run dev              # Start development server
npm run deploy           # Deploy to default (dev)
npm run deploy:staging   # Deploy to staging
npm run deploy:production # Deploy to production
```

### Database
```bash
npm run db:migrate:dev        # Run migrations locally
npm run db:migrate:production # Run migrations on production
npm run db:query:dev -- "SELECT ..." # Query local DB
wrangler d1 backup qhub-production --output backup.sql # Backup
```

### Monitoring
```bash
wrangler tail                 # View real-time logs
wrangler tail --status error  # View only errors
wrangler deployments list     # List deployments
```

## 🔗 API Endpoints Quick Reference

### Health & Info
```bash
GET  /                   # API info
GET  /health            # Health check
GET  /version           # Version info
```

### Authentication
```bash
POST   /auth/register              # Register new user
POST   /auth/login                 # Login
POST   /auth/logout                # Logout current session
POST   /auth/logout-all            # Logout all sessions
GET    /auth/verify                # Verify token (🔒)
GET    /auth/sessions              # List sessions (🔒)
DELETE /auth/sessions/:id          # Delete session (🔒)
```

### AI Chat
```bash
POST   /ai/chat                    # Send message (🔒)
GET    /ai/conversations           # List conversations (🔒)
GET    /ai/conversations/:id       # Get conversation (🔒)
DELETE /ai/conversations/:id       # Delete conversation (🔒)
GET    /ai/usage                   # Usage stats (🔒)
```

### Quantum Jobs
```bash
POST   /quantum/submit             # Submit job (🔒)
GET    /quantum/jobs               # List jobs (🔒)
GET    /quantum/jobs/:id           # Get job details (🔒)
DELETE /quantum/jobs/:id           # Cancel job (🔒)
GET    /quantum/stats              # Job statistics (🔒)
POST   /quantum/jobs/:id/rerun     # Rerun job (🔒)
```

🔒 = Requires authentication

## 💡 Common Tasks

### Register and Login
```bash
# Register
curl -X POST $API_URL/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"pass123"}'

# Extract token (with jq)
TOKEN=$(curl -X POST $API_URL/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"pass123"}' | jq -r '.token')
```

### AI Chat
```bash
curl -X POST $API_URL/ai/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"message":"Hello, explain quantum computing"}'
```

### Submit Quantum Job
```bash
curl -X POST $API_URL/quantum/submit \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"circuit_code":"qc = QuantumCircuit(2)","name":"Test"}'
```

## 📊 Usage Limits

### AI Chat (messages/day)
- Free: 10 messages
- Pro: 100 messages
- Enterprise: 1000 messages

### Quantum Jobs (concurrent)
- Free: 3 jobs
- Pro: 10 jobs
- Enterprise: 50 jobs

## ⚡ HTTP Status Codes

- `200` - Success
- `201` - Created
- `400` - Bad Request (invalid input)
- `401` - Unauthorized (missing/invalid token)
- `404` - Not Found
- `409` - Conflict (e.g., email exists)
- `429` - Too Many Requests (rate limit)
- `500` - Internal Server Error

## 🛠️ Troubleshooting

### Database Issues
```bash
# Check if DB exists
wrangler d1 list

# Re-run migrations
npm run db:migrate:dev

# Query to check users
npm run db:query:dev -- "SELECT COUNT(*) FROM users"
```

### Token Issues
```bash
# Check JWT_SECRET is set
wrangler secret list

# Set JWT_SECRET
wrangler secret put JWT_SECRET
```

### View Logs
```bash
# Real-time logs
wrangler tail

# Filter by POST requests
wrangler tail --method POST

# Filter errors
wrangler tail --status error
```

## 📁 File Locations

- **Routes**: `src/routes/`
- **Middleware**: `src/middleware/`
- **Types**: `src/types.ts`
- **Utils**: `src/utils.ts`
- **Config**: `wrangler.toml`
- **Schema**: `migrations/001_init_schema.sql`

## 🔐 Security Checklist

- [ ] JWT_SECRET is set and strong
- [ ] .dev.vars not committed
- [ ] Input validation on all endpoints
- [ ] Using parameterized queries
- [ ] Error messages don't leak sensitive data
- [ ] CORS configured appropriately

## 📚 Documentation

- **README.md** - User guide and setup
- **DEVELOPMENT.md** - Development guide
- **API_EXAMPLES.md** - Complete API examples
- **DEPLOYMENT_CHECKLIST.md** - Pre-deployment checklist
- **BUILD_SUMMARY.md** - What was built

## 🆘 Support

- Check documentation first
- Review `wrangler tail` for errors
- Query database to verify state
- Check Cloudflare dashboard
- Test locally before deploying

## 🎯 Production Deployment Flow

1. Test locally (`npm run dev`)
2. Deploy to staging (`npm run deploy:staging`)
3. Test staging thoroughly
4. Run production migrations (`npm run db:migrate:production`)
5. Deploy to production (`npm run deploy:production`)
6. Monitor logs (`wrangler tail --env production`)
7. Verify health check
8. Monitor for 24 hours

## 🔧 Environment Variables

### Required
- `JWT_SECRET` - JWT signing secret (32+ chars)

### Optional
- `JWT_EXPIRY_HOURS` - Token expiry (default: 24)
- `ENVIRONMENT` - Environment name (dev/staging/prod)

## ⚙️ Cloudflare Dashboard

Access at: https://dash.cloudflare.com/

Monitor:
- Workers analytics
- Request logs
- Error rates
- D1 database metrics

---

**Version**: 1.0.0  
**Last Updated**: 2024  
**Framework**: Hono + Cloudflare Workers  
**Language**: TypeScript
