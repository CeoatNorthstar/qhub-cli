# QHub CLI + Workers API - Complete Integration Guide

## 🎯 Architecture Overview

```
┌─────────────────┐      HTTP/REST       ┌──────────────────┐
│                 │ ────────────────────> │                  │
│   Rust CLI      │                       │  TypeScript API  │
│   (TUI Client)  │ <──────────────────── │  (Workers)       │
│                 │      JSON Responses   │                  │
└─────────────────┘                       └──────────────────┘
        │                                          │
        │ Local Config                             │
        ↓ (~/.qhub/)                                ↓
   config.toml                          ┌─────────────────────┐
   - api_url                            │  Cloudflare D1      │
   - token                              │  (SQLite Database)  │
   - user info                          └─────────────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ (`cargo --version`)
- Node.js 18+ (`node --version`)
- npm 9+ (`npm --version`)

### 1. Start the Backend

```bash
cd workers
npm install
npx wrangler d1 migrations apply qhub-dev --local
npm run dev
```

Backend will be available at: `http://localhost:8787`

### 2. Configure the CLI

```bash
# Set API URL environment variable
export QHUB_API_URL=http://localhost:8787

# Or create .env file in project root
echo "QHUB_API_URL=http://localhost:8787" > .env
```

### 3. Build and Run the CLI

```bash
cargo build --release
./target/release/qhub
```

## 📡 API Endpoints

### Authentication
- `POST /auth/register` - Create new account
- `POST /auth/login` - Authenticate user
- `POST /auth/logout` - Invalidate session
- `GET /auth/verify` - Verify JWT token

### AI Chat
- `POST /ai/chat` - Send message to AI (requires auth)
- `GET /ai/conversations` - List conversations
- `GET /ai/conversations/:id` - Get conversation details

### Quantum Jobs
- `POST /quantum/submit` - Submit quantum job
- `GET /quantum/jobs` - List user jobs
- `GET /quantum/jobs/:id` - Get job details

### Health
- `GET /` - API info
- `GET /health` - Health check

## 🔐 Authentication Flow

### Registration
```bash
# In CLI:
/register user@example.com myusername MySecurePass123!

# API Call:
POST /auth/register
{
  "email": "user@example.com",
  "username": "myusername",
  "password": "MySecurePass123!"
}

# Response:
{
  "token": "eyJhbGc...",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "myusername",
    "tier": "free"
  },
  "expires_at": 1234567890
}
```

### Login
```bash
# In CLI:
/login user@example.com MySecurePass123!

# API Call:
POST /auth/login
{
  "email": "user@example.com",
  "password": "MySecurePass123!"
}
```

### Token Storage
- CLI stores token in `~/.qhub/config.toml`
- Token automatically included in all API requests
- Token validated on CLI startup
- Expired tokens automatically cleared

## 🧪 Testing

### Run Integration Tests
```bash
./test_integration.sh
```

Tests verify:
1. Backend health check
2. User registration
3. Token verification
4. AI chat functionality
5. Logout
6. CLI build

### Manual Testing
```bash
# Test backend directly
curl http://localhost:8787/health

# Test registration
curl -X POST http://localhost:8787/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"Test123!","username":"testuser"}'

# Test AI chat (with token)
curl -X POST http://localhost:8787/ai/chat \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"message":"What is quantum computing?","conversation_id":null}'
```

## 📁 File Structure

```
qhub-cli/
├── src/
│   ├── api/
│   │   ├── client.rs        # HTTP API client (reqwest)
│   │   └── mod.rs
│   ├── tui/
│   │   ├── app.rs           # TUI app (uses ApiClient)
│   │   ├── ui.rs
│   │   └── input.rs
│   ├── config/
│   │   └── settings.rs      # Config with api_url
│   └── main.rs
├── workers/
│   ├── src/
│   │   ├── index.ts         # Main Hono app
│   │   ├── types.ts         # TypeScript types
│   │   ├── utils.ts         # JWT, bcrypt, helpers
│   │   ├── middleware/
│   │   │   └── auth.ts      # Auth middleware
│   │   └── routes/
│   │       ├── auth.ts      # Auth endpoints
│   │       ├── ai.ts        # AI endpoints
│   │       └── quantum.ts   # Quantum endpoints
│   ├── migrations/
│   │   └── 001_init_schema.sql
│   ├── wrangler.toml        # Workers config
│   └── package.json
├── test_integration.sh      # Integration tests
└── .env                     # Local environment config
```

## 🔄 Development Workflow

### Local Development
1. Start backend: `cd workers && npm run dev`
2. Set API URL: `export QHUB_API_URL=http://localhost:8787`
3. Run CLI: `cargo run`

### Making Changes

**Backend Changes:**
```bash
cd workers
# Edit TypeScript files in src/
npm run dev  # Auto-reloads on changes
```

**CLI Changes:**
```bash
# Edit Rust files in src/
cargo build
./target/debug/qhub
```

### Database Migrations

**Add New Migration:**
```bash
cd workers/migrations
# Create 002_your_migration.sql
# Apply locally:
npx wrangler d1 migrations apply qhub-dev --local
# Apply to staging:
npx wrangler d1 migrations apply qhub-staging --remote
```

## 🚀 Deployment

### Deploy Backend to Cloudflare Workers

**Staging:**
```bash
cd workers
npx wrangler deploy --env staging
```

**Production:**
```bash
cd workers
npx wrangler deploy --env production
```

### Configure CLI for Production
```bash
# Set production API URL
export QHUB_API_URL=https://qhub-api.yourdomain.workers.dev

# Or update ~/.qhub/config.toml:
api_url = "https://qhub-api.yourdomain.workers.dev"
```

### Distribution
```bash
# Build release binary
cargo build --release

# Binary location:
./target/release/qhub

# Install system-wide (macOS/Linux):
sudo cp ./target/release/qhub /usr/local/bin/
```

## 🔧 Configuration

### Environment Variables

**CLI (.env file):**
```bash
QHUB_API_URL=http://localhost:8787  # API base URL
```

**Backend (.dev.vars file):**
```bash
JWT_SECRET=your-secret-key-min-32-chars
JWT_EXPIRY_HOURS=24
```

### Config File (~/.qhub/config.toml)
```toml
version = 1
api_url = "http://localhost:8787"

[user]
email = "user@example.com"
token = "eyJhbGc..."
tier = "free"

[ai]
provider = "cloudflare"
model = "llama-2-7b-chat"
max_tokens = 2000

[ui]
theme = "dark"
color_scheme = "cyan"
```

## 📊 Monitoring

### Backend Logs
```bash
# Local development logs
cd workers && npm run dev
# Logs appear in terminal

# Production logs (Cloudflare dashboard)
# Visit: https://dash.cloudflare.com → Workers → Your worker → Logs
```

### CLI Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug cargo run
```

## 🐛 Troubleshooting

### Backend Not Starting
```bash
# Check if port 8787 is in use
lsof -i :8787
# Kill conflicting process if needed

# Verify migrations applied
cd workers
npx wrangler d1 migrations list qhub-dev --local
```

### CLI Can't Connect
```bash
# Verify backend is running
curl http://localhost:8787/health

# Check API URL in config
cat ~/.qhub/config.toml | grep api_url

# Verify environment variable
echo $QHUB_API_URL
```

### Authentication Failures
```bash
# Clear stored session
rm ~/.qhub/config.toml

# Restart CLI and register again
cargo run
# Then: /register email username password
```

### Database Issues
```bash
# Reset local database
cd workers
rm -rf .wrangler/state/v3/d1
npx wrangler d1 migrations apply qhub-dev --local
```

## 📚 Additional Resources

- **Backend API Documentation**: `workers/README.md`
- **API Examples**: `workers/API_EXAMPLES.md`
- **Development Guide**: `workers/DEVELOPMENT.md`
- **Deployment Checklist**: `workers/DEPLOYMENT_CHECKLIST.md`

## 🎯 Next Steps

1. **Add Rate Limiting**: Implement request throttling
2. **Add Caching**: Cache AI responses for common queries
3. **Add Analytics**: Track usage metrics
4. **Add WebSocket Support**: Real-time updates for quantum jobs
5. **Add OAuth**: Support GitHub/Google login
6. **Add CI/CD**: Automate testing and deployment

---

**Built with Enterprise Best Practices** ✨
- Clean separation of concerns
- Type-safe APIs (TypeScript + Rust)
- Comprehensive error handling
- Secure authentication (JWT + bcrypt)
- Production-ready infrastructure
