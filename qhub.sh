#!/bin/bash
# Enterprise QHub Launcher
# Handles all environment setup and graceful cleanup

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}"
cat << 'EOF'
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   ██████╗ ██╗  ██╗██╗   ██╗██████╗                               ║
║  ██╔═══██╗██║  ██║██║   ██║██╔══██╗                              ║
║  ██║   ██║███████║██║   ██║██████╔╝                              ║
║  ██║▄▄ ██║██╔══██║██║   ██║██╔══██╗                              ║
║  ╚██████╔╝██║  ██║╚██████╔╝██████╔╝                              ║
║   ╚══▀▀═╝ ╚═╝  ╚═╝ ╚═════╝ ╚═════╝                               ║
║                                                                   ║
║   Quantum Computing + AI - Enterprise CLI                         ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"

echo -e "${YELLOW}🔧 Pre-flight checks...${NC}"
echo ""

# Check for .env file
if [ ! -f .env ]; then
    echo -e "${YELLOW}⚠️  No .env file found. Creating from template...${NC}"
    if [ -f .env.local ]; then
        cp .env.local .env
        echo -e "${GREEN}✅ Created .env from .env.local${NC}"
    elif [ -f .env.example ]; then
        cp .env.example .env
        echo -e "${YELLOW}⚠️  Created .env from example. Please configure DATABASE_URL.${NC}"
    else
        cat > .env << 'ENVFILE'
DATABASE_URL=postgres://postgres:devpass@localhost:5432/app
JWT_SECRET=development-secret-key-change-in-production
ENVFILE
        echo -e "${GREEN}✅ Created default .env file${NC}"
    fi
fi

# Load .env
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
    echo -e "${GREEN}✅ Loaded environment variables${NC}"
fi

# Check database
echo -e "${YELLOW}🔍 Checking database connection...${NC}"
if docker ps | grep -q pg-local; then
    echo -e "${GREEN}✅ PostgreSQL container is running${NC}"
    
    # Test connection
    if docker exec pg-local psql -U postgres -d app -c "SELECT 1" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Database connection successful${NC}"
    else
        echo -e "${RED}❌ Database connection failed${NC}"
        echo -e "${YELLOW}💡 Try: docker-compose up -d${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  PostgreSQL container not running${NC}"
    echo -e "${YELLOW}💡 Starting database...${NC}"
    if [ -f docker-compose.yml ]; then
        docker-compose up -d
        echo -e "${GREEN}✅ Database started${NC}"
        sleep 2
    else
        echo -e "${RED}❌ docker-compose.yml not found${NC}"
        exit 1
    fi
fi

echo ""
echo -e "${YELLOW}🏗️  Building application...${NC}"
cargo build --quiet 2>/dev/null || cargo build

echo ""
echo -e "${GREEN}✅ Ready to launch!${NC}"
echo ""
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}📚 Quick Start Guide:${NC}"
echo ""
echo "  First time user:"
echo "    /register <email> <username> <password>"
echo ""
echo "  Returning user:"
echo "    /login <email> <password>"
echo ""
echo "  After login:"
echo "    \"Create a Bell state quantum circuit\""
echo "    /status"
echo "    /help"
echo ""
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${YELLOW}🚀 Launching QHub...${NC}"
echo ""

# Set up terminal cleanup trap
cleanup() {
    echo ""
    echo -e "${YELLOW}🧹 Cleaning up...${NC}"
    # Reset terminal
    stty sane 2>/dev/null || true
    tput reset 2>/dev/null || true
    echo -e "${GREEN}✅ Terminal restored${NC}"
    echo -e "${CYAN}👋 Goodbye from QHub!${NC}"
}

trap cleanup EXIT INT TERM

# Run the app
cargo run

# Explicit cleanup
cleanup
