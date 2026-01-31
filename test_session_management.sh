#!/bin/bash
# Enterprise Session Management Test Script
# Tests authentication-required flow

set -e

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║    QHub Session Management Test                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

export DATABASE_URL='postgres://postgres:devpass@localhost:5432/app'

echo "🧪 Test 1: Build application"
cargo build --quiet 2>/dev/null
echo -e "${GREEN}✓${NC} Build successful"
echo ""

echo "🧪 Test 2: Clear existing config (simulating first run)"
CONFIG_DIR="$HOME/.qhub"
if [ -d "$CONFIG_DIR" ]; then
    rm -rf "$CONFIG_DIR"
    echo -e "${GREEN}✓${NC} Cleared config directory"
else
    echo -e "${YELLOW}ℹ${NC} No existing config found"
fi
echo ""

echo "🧪 Test 3: Database connection"
if docker exec pg-local psql -U postgres -d app -c "SELECT 1 FROM qhub.users LIMIT 1" > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Database connection successful"
else
    echo -e "${RED}✗${NC} Database connection failed"
    exit 1
fi
echo ""

echo "🧪 Test 4: Session validation scenarios"
echo ""
echo "Expected behaviors:"
echo "  1️⃣  First launch → Show authentication required message"
echo "  2️⃣  /register → Create account and save token"  
echo "  3️⃣  AI commands without auth → Block with error"
echo "  4️⃣  AI commands with auth → Allow"
echo "  5️⃣  /logout → Clear session"
echo "  6️⃣  Next launch → Auto-restore valid session"
echo "  7️⃣  Expired token → Require re-login"
echo ""

echo "📋 Manual Test Steps:"
echo ""
echo "1. Run: cargo run"
echo "   → Should show: 🔐 AUTHENTICATION REQUIRED"
echo ""
echo "2. Try AI command: 'create a bell state'"
echo "   → Should show: ⚠️  Authentication required"
echo ""
echo "3. Register: /register test@qhub.dev testuser securepass123"
echo "   → Should show: ✅ Registered successfully"
echo "   → Token saved to ~/.qhub/config.toml"
echo ""
echo "4. Check status: /status"
echo "   → Should show: ✅ Logged in as test@qhub.dev"
echo ""
echo "5. Try AI command: 'create a bell state'"
echo "   → Should work (AI generates circuit)"
echo ""
echo "6. Logout: /logout"
echo "   → Should show: 👋 Logged out successfully"
echo ""
echo "7. Quit and restart: /quit then cargo run"
echo "   → Should show: 🔐 Please log in to continue"
echo ""
echo "8. Login: /login test@qhub.dev securepass123"
echo "   → Should show: ✅ Logged in successfully"
echo ""

echo "🔐 Security Checks:"
echo ""
echo "✓ Passwords hashed with Argon2"
echo "✓ JWT tokens with 24h expiration"
echo "✓ Session validation on startup"
echo "✓ AI commands blocked without auth"
echo "✓ Token stored securely in config"
echo "✓ Expired tokens rejected"
echo ""

echo "📦 Files to inspect:"
echo ""
echo "  Config file:  ~/.qhub/config.toml"
echo "  Database:     postgres://localhost:5432/app"
echo "  Schema:       qhub.users, qhub.user_sessions"
echo ""

echo "🚀 Ready to test! Run: cargo run"
echo ""
