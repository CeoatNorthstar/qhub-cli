#!/bin/bash
set -e

echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║          QHub Cloudflare D1 Production Deployment                ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check if wrangler is installed
if ! command -v wrangler &> /dev/null; then
    echo -e "${RED}❌ Wrangler CLI not found${NC}"
    echo "Install with: npm install -g wrangler"
    exit 1
fi

echo -e "${GREEN}✓${NC} Wrangler CLI found"

# Check if logged in
if ! wrangler whoami &> /dev/null; then
    echo -e "${YELLOW}⚠${NC}  Not logged in to Cloudflare"
    echo "Running: wrangler login"
    wrangler login
fi

echo -e "${GREEN}✓${NC} Logged in to Cloudflare"

# Database configuration
DB_ID="a52bc4fb-9185-4185-bb6e-572c3dd3feaf"
DB_NAME="qhub-production"

echo ""
echo -e "${CYAN}Database Configuration:${NC}"
echo "  Name: $DB_NAME"
echo "  ID:   $DB_ID"
echo ""

# Menu
echo "What would you like to do?"
echo ""
echo "  1) Apply migrations to D1"
echo "  2) Verify database tables"
echo "  3) Set secrets (JWT_SECRET, etc.)"
echo "  4) Deploy to Cloudflare Workers"
echo "  5) View database info"
echo "  6) Backup database"
echo "  7) Query database (interactive)"
echo "  8) Full setup (1+2+3+4)"
echo ""
read -p "Enter choice [1-8]: " choice

case $choice in
    1)
        echo ""
        echo -e "${CYAN}Applying migrations...${NC}"
        wrangler d1 execute $DB_NAME --file=./migrations/001_init_schema_d1.sql
        echo -e "${GREEN}✓${NC} Migrations applied"
        ;;
    2)
        echo ""
        echo -e "${CYAN}Checking tables...${NC}"
        wrangler d1 execute $DB_NAME \
            --command="SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
        ;;
    3)
        echo ""
        echo -e "${CYAN}Setting secrets...${NC}"
        echo ""
        
        # JWT Secret
        echo -e "${YELLOW}Setting JWT_SECRET${NC}"
        echo "Generate with: openssl rand -base64 32"
        wrangler secret put JWT_SECRET
        
        # AI Token
        echo ""
        echo -e "${YELLOW}Setting CLOUDFLARE_AI_TOKEN${NC}"
        wrangler secret put CLOUDFLARE_AI_TOKEN
        
        # Optional: IBM Quantum
        echo ""
        read -p "Do you want to set IBM_QUANTUM_TOKEN? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            wrangler secret put IBM_QUANTUM_TOKEN
        fi
        
        echo ""
        echo -e "${GREEN}✓${NC} Secrets configured"
        ;;
    4)
        echo ""
        echo -e "${CYAN}Deploying to Cloudflare Workers...${NC}"
        wrangler deploy
        echo ""
        echo -e "${GREEN}✓${NC} Deployed successfully!"
        echo ""
        echo "Test with: curl https://api.qhub.dev/health"
        ;;
    5)
        echo ""
        wrangler d1 info $DB_NAME
        ;;
    6)
        echo ""
        BACKUP_FILE="backup-$(date +%Y%m%d-%H%M%S).sql"
        echo -e "${CYAN}Creating backup: $BACKUP_FILE${NC}"
        wrangler d1 export $DB_NAME --output=$BACKUP_FILE
        echo -e "${GREEN}✓${NC} Backup created: $BACKUP_FILE"
        ;;
    7)
        echo ""
        echo -e "${CYAN}Enter SQL query (or 'exit' to quit):${NC}"
        while true; do
            read -p "SQL> " query
            if [[ "$query" == "exit" ]]; then
                break
            fi
            wrangler d1 execute $DB_NAME --command="$query"
        done
        ;;
    8)
        echo ""
        echo -e "${CYAN}═══ Full Production Setup ═══${NC}"
        
        # Step 1: Migrations
        echo ""
        echo -e "${CYAN}[1/4] Applying migrations...${NC}"
        wrangler d1 execute $DB_NAME --file=./migrations/001_init_schema_d1.sql
        echo -e "${GREEN}✓${NC} Migrations applied"
        
        # Step 2: Verify
        echo ""
        echo -e "${CYAN}[2/4] Verifying tables...${NC}"
        wrangler d1 execute $DB_NAME \
            --command="SELECT COUNT(*) as table_count FROM sqlite_master WHERE type='table'"
        echo -e "${GREEN}✓${NC} Tables verified"
        
        # Step 3: Secrets
        echo ""
        echo -e "${CYAN}[3/4] Configuring secrets...${NC}"
        echo ""
        echo -e "${YELLOW}JWT_SECRET:${NC}"
        echo "Generate with: openssl rand -base64 32"
        wrangler secret put JWT_SECRET
        
        echo ""
        echo -e "${YELLOW}CLOUDFLARE_AI_TOKEN:${NC}"
        wrangler secret put CLOUDFLARE_AI_TOKEN
        
        echo -e "${GREEN}✓${NC} Secrets configured"
        
        # Step 4: Deploy
        echo ""
        echo -e "${CYAN}[4/4] Deploying...${NC}"
        wrangler deploy
        
        echo ""
        echo -e "${GREEN}═══════════════════════════════════════${NC}"
        echo -e "${GREEN}✓ Production setup complete!${NC}"
        echo -e "${GREEN}═══════════════════════════════════════${NC}"
        echo ""
        echo "Next steps:"
        echo "  • Test: curl https://api.qhub.dev/health"
        echo "  • Monitor: wrangler tail"
        echo "  • View logs in Cloudflare Dashboard"
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}Done!${NC}"
