#!/bin/bash
# Auto-detect current branch and copy appropriate environment config

set -e

BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")

case "$BRANCH" in
  main)
    if [ -f .env.production ]; then
      cp .env.production .env
      echo "✅ Configured for PRODUCTION (main branch)"
      echo "   API: https://qhub-api-production.a-contactnaol.workers.dev"
      echo "   DB:  b607a2f3-0ab5-407e-b63a-eb21d01084d0"
    else
      echo "❌ ERROR: .env.production not found!"
      exit 1
    fi
    ;;
    
  staging)
    if [ -f .env.staging ]; then
      cp .env.staging .env
      echo "✅ Configured for STAGING (staging branch)"
      echo "   API: https://qhub-api-staging.a-contactnaol.workers.dev"
      echo "   DB:  18a877b3-4dd2-4f07-bf92-2b341bf8a2ba"
    else
      echo "❌ ERROR: .env.staging not found!"
      exit 1
    fi
    ;;
    
  *)
    echo "⚠️  Unknown branch: $BRANCH"
    echo "   Defaulting to local development config"
    if [ ! -f .env ]; then
      echo "QHUB_API_URL=http://localhost:8787" > .env
      echo "ENVIRONMENT=development" >> .env
    fi
    echo "   API: http://localhost:8787"
    ;;
esac
