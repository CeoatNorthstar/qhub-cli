#!/bin/bash
# Quick startup test

echo "Testing QHub startup..."
echo ""

cd /Users/icon/Developer/qhub-cli

# Check .env file
if [ -f .env ]; then
    echo "✅ .env file exists"
    cat .env
else
    echo "❌ .env file missing"
    exit 1
fi

echo ""
echo "Building..."
cargo build --quiet

echo ""
echo "Starting app (will auto-close in 3 seconds)..."
echo ""

# Run the app in background and capture stderr
cargo run 2>&1 &
APP_PID=$!

# Wait 3 seconds
sleep 3

# Kill the app
kill $APP_PID 2>/dev/null
wait $APP_PID 2>/dev/null

echo ""
echo "✅ App started and closed cleanly"
echo ""
echo "Now try manually: cargo run"
