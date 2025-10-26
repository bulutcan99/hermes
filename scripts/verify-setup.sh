#!/bin/bash

# Hermes Setup Verification Script
# This script verifies that Sprint 0 setup is complete

set -e

echo "🚀 Hermes Sprint 0 Verification"
echo "================================"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check function
check() {
	if [ $? -eq 0 ]; then
		echo -e "${GREEN}✓${NC} $1"
	else
		echo -e "${RED}✗${NC} $1"
		exit 1
	fi
}

# 1. Check Rust installation
echo "Checking Rust installation..."
rustc --version >/dev/null 2>&1
check "Rust compiler found"

cargo --version >/dev/null 2>&1
check "Cargo found"

# 2. Check Docker
echo ""
echo "Checking Docker..."
docker --version >/dev/null 2>&1
check "Docker found"

docker-compose --version >/dev/null 2>&1 || docker compose version >/dev/null 2>&1
check "Docker Compose found"

# 3. Check workspace structure
echo ""
echo "Checking project structure..."
[ -f "Cargo.toml" ]
check "Workspace Cargo.toml exists"

[ -d "crates/common" ]
check "Common crate directory exists"

[ -f "docker-compose.yml" ]
check "Docker Compose file exists"

[ -f "infra/postgres/init.sql" ]
check "Database schema exists"

# 4. Build common crate
echo ""
echo "Building common crate..."
cargo build -p common >/dev/null 2>&1
check "Common crate builds successfully"

# 5. Run common crate tests
echo ""
echo "Running common crate tests..."
cargo test -p common >/dev/null 2>&1
check "Common crate tests pass"

# 6. Check Docker services
echo ""
echo "Checking Docker infrastructure..."

# Start services if not running
docker-compose up -d postgres redis nats >/dev/null 2>&1

# Wait for services
echo "Waiting for services to be healthy..."
sleep 10

# Check Postgres
docker-compose ps postgres | grep -q "Up"
check "PostgreSQL is running"

# Check Redis
docker-compose ps redis | grep -q "Up"
check "Redis is running"

# Check NATS
docker-compose ps nats | grep -q "Up"
check "NATS is running"

# 7. Test database connection
echo ""
echo "Testing database connection..."
PGPASSWORD=hermes_dev psql -h localhost -U hermes -d hermes -c "SELECT 1;" >/dev/null 2>&1
check "PostgreSQL connection successful"

# 8. Test Redis connection
echo ""
echo "Testing Redis connection..."
redis-cli -h localhost ping >/dev/null 2>&1
check "Redis connection successful"

# 9. Test NATS connection
echo ""
echo "Testing NATS connection..."
curl -s http://localhost:8222/healthz >/dev/null 2>&1
check "NATS is healthy"

# 10. Verify database schema
echo ""
echo "Verifying database schema..."
TABLE_COUNT=$(PGPASSWORD=hermes_dev psql -h localhost -U hermes -d hermes -tAc "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema='public' AND table_type='BASE TABLE';")

if [ "$TABLE_COUNT" -ge 5 ]; then
	echo -e "${GREEN}✓${NC} Database schema loaded ($TABLE_COUNT tables)"
else
	echo -e "${RED}✗${NC} Database schema incomplete ($TABLE_COUNT tables found, expected at least 5)"
	exit 1
fi

# 11. Check for sample data
echo ""
echo "Checking sample data..."
VEHICLE_COUNT=$(PGPASSWORD=hermes_dev psql -h localhost -U hermes -d hermes -tAc "SELECT COUNT(*) FROM vehicles;")

if [ "$VEHICLE_COUNT" -ge 1 ]; then
	echo -e "${GREEN}✓${NC} Sample data loaded ($VEHICLE_COUNT vehicles)"
else
	echo -e "${YELLOW}⚠${NC} No sample data found (this is okay)"
fi

# 12. Check formatting
echo ""
echo "Checking code formatting..."
cargo fmt --all -- --check >/dev/null 2>&1
check "Code is properly formatted"

# 13. Run clippy
echo ""
echo "Running clippy lints..."
cargo clippy --all-targets --all-features -- -D warnings >/dev/null 2>&1
check "Clippy checks pass"

# Summary
echo ""
echo "================================"
echo -e "${GREEN}✓ Sprint 0 verification complete!${NC}"
echo ""
echo "Next steps:"
echo "  1. Review the README.md"
echo "  2. Read docs/DEVELOPMENT.md"
echo "  3. Say 'devam' to proceed to Sprint 1"
echo ""
echo "Quick commands:"
echo "  make help       - See all available commands"
echo "  make test       - Run all tests"
echo "  make check      - Run full quality checks"
echo ""
