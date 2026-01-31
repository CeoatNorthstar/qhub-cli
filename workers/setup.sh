#!/bin/bash
# QHub Workers - Quick Setup Script

set -e

echo "🚀 QHub Workers - Setup Script"
echo "================================"
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js 18+ first."
    exit 1
fi

echo -e "${GREEN}✓${NC} Node.js $(node --version) detected"

# Check if wrangler is installed
if ! command -v wrangler &> /dev/null; then
    echo -e "${YELLOW}⚠${NC} Wrangler not found. Installing globally..."
    npm install -g wrangler
fi

echo -e "${GREEN}✓${NC} Wrangler detected"

# Install dependencies
echo ""
echo -e "${BLUE}📦 Installing dependencies...${NC}"
npm install

# Check if logged in to Cloudflare
echo ""
echo -e "${BLUE}🔐 Checking Cloudflare authentication...${NC}"
if ! wrangler whoami &> /dev/null; then
    echo -e "${YELLOW}⚠${NC} Not logged in to Cloudflare"
    echo -e "${BLUE}  Run: wrangler login${NC}"
else
    echo -e "${GREEN}✓${NC} Logged in to Cloudflare"
fi

# Instructions
echo ""
echo -e "${GREEN}✅ Setup complete!${NC}"
echo ""
echo "📋 Next steps:"
echo ""
echo "1. Create D1 databases:"
echo -e "   ${BLUE}wrangler d1 create qhub-dev${NC}"
echo ""
echo "2. Update wrangler.toml with database IDs"
echo ""
echo "3. Run migrations:"
echo -e "   ${BLUE}npm run db:migrate:dev${NC}"
echo ""
echo "4. Set JWT secret:"
echo -e "   ${BLUE}wrangler secret put JWT_SECRET${NC}"
echo "   (Enter a random 32+ character string)"
echo ""
echo "5. Start development server:"
echo -e "   ${BLUE}npm run dev${NC}"
echo ""
echo "📚 See README.md for more information"
echo ""
