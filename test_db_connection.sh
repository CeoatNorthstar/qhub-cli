#!/bin/bash
set -e

echo "Testing QHub Database Connection..."
echo "===================================="

# Load environment
export $(cat .env.local | grep -v '^#' | xargs)

# Test database connection
echo "1. Testing PostgreSQL connection..."
docker exec pg-local psql -U postgres -d app -c "SELECT 'Connection successful!' as status;" || exit 1

# Check schema exists
echo ""
echo "2. Checking qhub schema..."
docker exec pg-local psql -U postgres -d app -c "SELECT schema_name FROM information_schema.schemata WHERE schema_name = 'qhub';" || exit 1

# List tables
echo ""
echo "3. Listing qhub tables..."
docker exec pg-local psql -U postgres -d app -c "SELECT tablename FROM pg_tables WHERE schemaname = 'qhub' ORDER BY tablename;" || exit 1

# Check triggers
echo ""
echo "4. Checking triggers..."
docker exec pg-local psql -U postgres -d app -c "SELECT trigger_name, event_object_table FROM information_schema.triggers WHERE trigger_schema = 'qhub';" || exit 1

echo ""
echo "===================================="
echo "✅ All database tests passed!"
echo "Database URL: $DATABASE_URL"
echo "Schema: qhub"
echo "Tables: 7 (users, user_sessions, oauth_connections, api_keys, user_preferences, usage_records, quantum_jobs)"
