#!/bin/bash
# QHub Deployment Script - Enterprise Grade with Branch Protection
# Automates deployment to staging and production environments
# Prevents deploying wrong environment from wrong branch

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "╔════════════════════════════════════════════════╗"
echo "║                                                ║"
echo "║        QHub Enterprise Deployment              ║"
echo "║        Branch-Protected Configuration          ║"
echo "║                                                ║"
echo "╚════════════════════════════════════════════════╝"
echo -e "${NC}"

# Get current Git branch
CURRENT_BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")

echo -e "${CYAN}📍 Current Branch: ${CURRENT_BRANCH}${NC}"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to prompt for confirmation
confirm() {
    read -p "$1 (y/N): " response
    case "$response" in
        [yY][eE][sS]|[yY]) 
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

# Check prerequisites
echo -e "${YELLOW}📋 Checking prerequisites...${NC}"
echo ""

if ! command_exists "npx"; then
    echo -e "${RED}❌ npx not found. Please install Node.js 18+${NC}"
    exit 1
fi

if ! command_exists "cargo"; then
    echo -e "${RED}❌ cargo not found. Please install Rust${NC}"
    exit 1
fi

echo -e "${GREEN}✅ All prerequisites met${NC}"
echo ""

# Select environment
echo -e "${BLUE}🎯 Select deployment environment:${NC}"
echo "  1) Staging (qhub-api-staging.workers.dev)"
echo "  2) Production (qhub-api-production.workers.dev)"
echo "  3) Both (Staging first, then Production)"
echo ""
read -p "Enter choice (1-3): " ENV_CHOICE

case $ENV_CHOICE in
    1)
        DEPLOY_STAGING=true
        DEPLOY_PRODUCTION=false
        ;;
    2)
        DEPLOY_STAGING=false
        DEPLOY_PRODUCTION=true
        ;;
    3)
        DEPLOY_STAGING=true
        DEPLOY_PRODUCTION=true
        ;;
    *)
        echo -e "${RED}❌ Invalid choice${NC}"
        exit 1
        ;;
esac

echo ""

# ============================================================================
# BRANCH PROTECTION - Validate branch vs environment
# ============================================================================
echo -e "${YELLOW}🔒 Branch Protection Check...${NC}"

# Production deployment requires main branch
if [ "$DEPLOY_PRODUCTION" = true ] && [ "$CURRENT_BRANCH" != "main" ]; then
    echo -e "${RED}╔════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║  ⛔ BRANCH PROTECTION VIOLATION                ║${NC}"
    echo -e "${RED}╚════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${RED}❌ Cannot deploy PRODUCTION from '${CURRENT_BRANCH}' branch${NC}"
    echo ""
    echo "Production deployments are only allowed from 'main' branch."
    echo "This prevents accidentally using staging configs in production."
    echo ""
    echo "To deploy production:"
    echo "  1. Merge your changes to main: git checkout main && git merge staging"
    echo "  2. Run deploy script from main: git checkout main && ./deploy.sh"
    echo ""
    exit 1
fi

# Staging deployment should be from staging branch
if [ "$DEPLOY_STAGING" = true ] && [ "$CURRENT_BRANCH" = "main" ]; then
    echo -e "${YELLOW}⚠️  WARNING: Deploying STAGING from 'main' branch${NC}"
    echo ""
    echo "Staging is typically deployed from 'staging' branch."
    echo "Are you sure you want to deploy staging from main?"
    echo ""
    if ! confirm "Continue anyway?"; then
        echo -e "${YELLOW}Deployment cancelled${NC}"
        exit 0
    fi
fi

echo -e "${GREEN}✅ Branch protection check passed${NC}"
echo ""

# Deploy to Staging
if [ "$DEPLOY_STAGING" = true ]; then
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}  STAGING DEPLOYMENT${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    # Step 1: Check if staging database exists
    echo -e "${BLUE}1️⃣  Checking staging database...${NC}"
    cd workers
    
    # Verify staging database ID in wrangler.toml
    STAGING_DB_ID=$(grep -A 3 "\[env.staging.d1_databases\]" wrangler.toml | grep "database_id" | cut -d'"' -f2)
    EXPECTED_STAGING_DB="18a877b3-4dd2-4f07-bf92-2b341bf8a2ba"
    
    if [ "$STAGING_DB_ID" != "$EXPECTED_STAGING_DB" ]; then
        echo -e "${RED}❌ Database ID mismatch!${NC}"
        echo "   Expected: $EXPECTED_STAGING_DB"
        echo "   Found:    $STAGING_DB_ID"
        echo ""
        echo "This could mean staging config has been corrupted."
        echo "Check workers/wrangler.toml [env.staging.d1_databases]"
        exit 1
    fi
    
    if npx wrangler d1 list | grep -q "qhub-staging"; then
        echo -e "${GREEN}✅ Staging database exists (ID: $STAGING_DB_ID)${NC}"
    else
        echo -e "${YELLOW}⚠️  Staging database not found. Creating...${NC}"
        echo ""
        echo "Run this command to create staging database:"
        echo "  npx wrangler d1 create qhub-staging"
        echo ""
        echo "Then copy the database_id from the output and update:"
        echo "  workers/wrangler.toml line 23"
        echo ""
        if confirm "Create database now?"; then
            npx wrangler d1 create qhub-staging
            echo ""
            echo -e "${YELLOW}📝 Please copy the database_id from above and update wrangler.toml${NC}"
            echo -e "${YELLOW}📝 Then run ./deploy.sh again${NC}"
        fi
        exit 1
    fi
    
    # Step 2: Check secrets
    echo ""
    echo -e "${BLUE}2️⃣  Checking secrets...${NC}"
    echo -e "${YELLOW}⚠️  You need to set JWT_SECRET for staging${NC}"
    echo ""
    if confirm "Have you set JWT_SECRET for staging? (wrangler secret put JWT_SECRET --env staging)"; then
        echo -e "${GREEN}✅ Secrets confirmed${NC}"
    else
        echo -e "${YELLOW}📝 Run this command first:${NC}"
        echo "   cd workers && npx wrangler secret put JWT_SECRET --env staging"
        echo ""
        if confirm "Run it now?"; then
            npx wrangler secret put JWT_SECRET --env staging
        else
            exit 1
        fi
    fi
    
    # Step 3: Apply migrations
    echo ""
    echo -e "${BLUE}3️⃣  Applying database migrations...${NC}"
    if confirm "Apply migrations to staging database?"; then
        npx wrangler d1 migrations apply qhub-staging --env staging --remote
        echo -e "${GREEN}✅ Migrations applied${NC}"
    else
        echo -e "${RED}❌ Skipping migrations - deployment may fail${NC}"
    fi
    
    # Step 4: Run tests
    echo ""
    echo -e "${BLUE}4️⃣  Running tests...${NC}"
    cd ..
    if ./test_integration.sh; then
        echo -e "${GREEN}✅ All tests passed${NC}"
    else
        echo -e "${RED}❌ Tests failed${NC}"
        if ! confirm "Continue deployment anyway?"; then
            exit 1
        fi
    fi
    cd workers
    
    # Step 5: Deploy
    echo ""
    echo -e "${BLUE}5️⃣  Deploying to Cloudflare Workers (Staging)...${NC}"
    if confirm "Deploy to staging now?"; then
        npx wrangler deploy --env staging
        echo ""
        echo -e "${GREEN}✅ Staging deployment complete!${NC}"
        echo ""
        echo -e "${YELLOW}📋 Staging Details:${NC}"
        echo "  URL: https://qhub-api-staging.workers.dev"
        echo "  Test: curl https://qhub-api-staging.workers.dev/health"
    else
        echo -e "${YELLOW}⚠️  Staging deployment skipped${NC}"
        exit 0
    fi
    
    # Test staging deployment
    echo ""
    echo -e "${BLUE}6️⃣  Testing staging deployment...${NC}"
    sleep 3  # Wait for Workers to propagate
    STAGING_URL="https://qhub-api-staging.workers.dev"
    
    if curl -s "$STAGING_URL/health" | grep -q "status"; then
        echo -e "${GREEN}✅ Staging is healthy!${NC}"
    else
        echo -e "${RED}❌ Staging health check failed${NC}"
        echo "   URL: $STAGING_URL/health"
        if ! confirm "Continue to production anyway?"; then
            exit 1
        fi
    fi
    
    cd ..
fi

# Deploy to Production
if [ "$DEPLOY_PRODUCTION" = true ]; then
    echo ""
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}  PRODUCTION DEPLOYMENT${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    echo -e "${RED}⚠️  WARNING: You are about to deploy to PRODUCTION${NC}"
    echo ""
    if ! confirm "Are you sure you want to deploy to production?"; then
        echo -e "${YELLOW}⚠️  Production deployment cancelled${NC}"
        exit 0
    fi
    
    cd workers
    
    # Step 1: Check production database
    echo ""
    echo -e "${BLUE}1️⃣  Checking production database...${NC}"
    
    # Verify production database ID in wrangler.toml
    PROD_DB_ID=$(grep -A 3 "\[env.production.d1_databases\]" wrangler.toml | grep "database_id" | cut -d'"' -f2)
    EXPECTED_PROD_DB="b607a2f3-0ab5-407e-b63a-eb21d01084d0"
    
    if [ "$PROD_DB_ID" != "$EXPECTED_PROD_DB" ]; then
        echo -e "${RED}❌ Database ID mismatch!${NC}"
        echo "   Expected: $EXPECTED_PROD_DB"
        echo "   Found:    $PROD_DB_ID"
        echo ""
        echo "⛔ CRITICAL: Production database ID is incorrect!"
        echo "This could mean production config has been corrupted."
        echo "Check workers/wrangler.toml [env.production.d1_databases]"
        echo ""
        echo "DO NOT PROCEED - Fix wrangler.toml first!"
        exit 1
    fi
    
    if npx wrangler d1 list | grep -q "qhub-production"; then
        echo -e "${GREEN}✅ Production database exists (ID: $PROD_DB_ID)${NC}"
    else
        echo -e "${RED}❌ Production database not found!${NC}"
        exit 1
    fi
    
    # Step 2: Check secrets
    echo ""
    echo -e "${BLUE}2️⃣  Checking secrets...${NC}"
    echo -e "${YELLOW}⚠️  You need to set JWT_SECRET for production${NC}"
    echo ""
    if confirm "Have you set JWT_SECRET for production? (wrangler secret put JWT_SECRET --env production)"; then
        echo -e "${GREEN}✅ Secrets confirmed${NC}"
    else
        echo -e "${YELLOW}📝 Run this command first:${NC}"
        echo "   cd workers && npx wrangler secret put JWT_SECRET --env production"
        echo ""
        if confirm "Run it now?"; then
            npx wrangler secret put JWT_SECRET --env production
        else
            exit 1
        fi
    fi
    
    # Step 3: Apply migrations
    echo ""
    echo -e "${BLUE}3️⃣  Applying database migrations...${NC}"
    if confirm "Apply migrations to production database?"; then
        npx wrangler d1 migrations apply qhub-production --env production --remote
        echo -e "${GREEN}✅ Migrations applied${NC}"
    else
        echo -e "${RED}❌ Skipping migrations - deployment may fail${NC}"
    fi
    
    # Step 4: Final confirmation
    echo ""
    echo -e "${RED}⚠️  FINAL CONFIRMATION${NC}"
    echo ""
    if ! confirm "Deploy to PRODUCTION now?"; then
        echo -e "${YELLOW}⚠️  Production deployment cancelled${NC}"
        exit 0
    fi
    
    # Step 5: Deploy
    echo ""
    echo -e "${BLUE}4️⃣  Deploying to Cloudflare Workers (Production)...${NC}"
    npx wrangler deploy --env production
    echo ""
    echo -e "${GREEN}✅ Production deployment complete!${NC}"
    echo ""
    echo -e "${YELLOW}📋 Production Details:${NC}"
    echo "  URL: https://qhub-api-production.workers.dev"
    echo "  Test: curl https://qhub-api-production.workers.dev/health"
    
    # Test production deployment
    echo ""
    echo -e "${BLUE}5️⃣  Testing production deployment...${NC}"
    sleep 3  # Wait for Workers to propagate
    PROD_URL="https://qhub-api-production.workers.dev"
    
    if curl -s "$PROD_URL/health" | grep -q "status"; then
        echo -e "${GREEN}✅ Production is healthy!${NC}"
    else
        echo -e "${RED}❌ Production health check failed${NC}"
        echo "   URL: $PROD_URL/health"
    fi
    
    cd ..
fi

# Build and distribute CLI
echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}  CLI BUILD & DISTRIBUTION${NC}"
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

if confirm "Build release CLI binary?"; then
    echo -e "${BLUE}🔨 Building release binary...${NC}"
    cargo build --release --quiet
    echo -e "${GREEN}✅ Binary built: target/release/qhub${NC}"
    echo ""
    
    # Show binary info
    BINARY_SIZE=$(du -h target/release/qhub | cut -f1)
    echo -e "${YELLOW}📊 Binary Info:${NC}"
    echo "  Size: $BINARY_SIZE"
    echo "  Location: target/release/qhub"
    echo ""
    
    # Optional: Install system-wide
    if confirm "Install CLI system-wide? (requires sudo)"; then
        sudo cp target/release/qhub /usr/local/bin/
        echo -e "${GREEN}✅ CLI installed to /usr/local/bin/qhub${NC}"
        echo ""
        echo "Test it:"
        echo "  qhub --version"
    fi
else
    echo -e "${YELLOW}⚠️  CLI build skipped${NC}"
fi

# Summary
echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}  🎉 DEPLOYMENT COMPLETE!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

if [ "$DEPLOY_STAGING" = true ]; then
    echo -e "${YELLOW}📋 Staging:${NC}"
    echo "  URL: https://qhub-api-staging.workers.dev"
    echo "  Dashboard: https://dash.cloudflare.com"
    echo ""
fi

if [ "$DEPLOY_PRODUCTION" = true ]; then
    echo -e "${YELLOW}📋 Production:${NC}"
    echo "  URL: https://qhub-api-production.workers.dev"
    echo "  Dashboard: https://dash.cloudflare.com"
    echo ""
fi

echo -e "${YELLOW}🔧 Configure CLI:${NC}"
echo "  export QHUB_API_URL=https://qhub-api-production.workers.dev"
echo "  # Or for staging: https://qhub-api-staging.workers.dev"
echo ""

echo -e "${YELLOW}📚 Next Steps:${NC}"
echo "  1. Test the deployed API with curl"
echo "  2. Configure CLI to use production URL"
echo "  3. Register a new account"
echo "  4. Monitor logs in Cloudflare dashboard"
echo "  5. Set up custom domain (optional)"
echo ""

echo -e "${GREEN}✨ Happy quantum computing!${NC}"
