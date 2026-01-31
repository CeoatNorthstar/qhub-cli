#!/bin/bash
# Quick Test Script for Auth System
# Run this to quickly verify the authentication system

set -e

export DATABASE_URL='postgres://postgres:devpass@localhost:5432/app'
export JWT_SECRET='development-secret-key-change-in-production'

echo "🧪 Quick Auth Test"
echo "=================="
echo ""

# Clean up
echo "🧹 Cleaning test data..."
docker exec pg-local psql -U postgres -d app -c "DELETE FROM qhub.users WHERE email = 'test@example.com';" > /dev/null 2>&1
echo "✓ Clean"
echo ""

# Verify build
echo "🔨 Checking build..."
if [ ! -f "target/debug/qhub" ]; then
    echo "Building..."
    cargo build --quiet
fi
echo "✓ Binary ready"
echo ""

# Instructions
echo "===================="
echo "✅ Ready to test!"
echo "===================="
echo ""
echo "Start the app:"
echo "  ./target/debug/qhub"
echo ""
echo "Then test these commands:"
echo "  1. /register test@example.com testuser testpass123"
echo "  2. /status"
echo "  3. /logout"
echo "  4. /login test@example.com testpass123"
echo "  5. /status"
echo ""
echo "Expected results:"
echo "  ✓ Registration succeeds and logs in"
echo "  ✓ Status shows email and tier"
echo "  ✓ Config saved to ~/.qhub/config.toml"
echo "  ✓ Logout clears user data"
echo "  ✓ Login succeeds with correct credentials"
echo ""
echo "Press Enter to start qhub..."
read

./target/debug/qhub
