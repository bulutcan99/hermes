.PHONY: help build test clean fmt clippy run-infra stop-infra logs check doc

# Default target
help:
	@echo "Hermes Development Commands"
	@echo "============================"
	@echo "make build        - Build all crates"
	@echo "make test         - Run all tests"
	@echo "make clean        - Clean build artifacts"
	@echo "make fmt          - Format code"
	@echo "make clippy       - Run linter"
	@echo "make check        - Run fmt + clippy + test"
	@echo "make doc          - Generate documentation"
	@echo "make run-infra    - Start infrastructure (Postgres, NATS, Redis)"
	@echo "make stop-infra   - Stop infrastructure"
	@echo "make logs         - Show infrastructure logs"
	@echo "make db-shell     - Connect to PostgreSQL"
	@echo "make nats-sub     - Subscribe to all NATS events"
	@echo "make coverage     - Generate test coverage report"

# Build targets
build:
	@echo "Building workspace..."
	cargo build --workspace

build-release:
	@echo "Building release..."
	cargo build --workspace --release

# Test targets
test:
	@echo "Running tests..."
	cargo test --workspace

test-verbose:
	@echo "Running tests with output..."
	cargo test --workspace -- --nocapture

test-integration:
	@echo "Running integration tests..."
	cargo test --workspace --test '*'

# Code quality
fmt:
	@echo "Formatting code..."
	cargo fmt --all

fmt-check:
	@echo "Checking formatting..."
	cargo fmt --all -- --check

clippy:
	@echo "Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

clippy-fix:
	@echo "Auto-fixing clippy issues..."
	cargo clippy --all-targets --all-features --fix

check: fmt-check clippy test
	@echo "All checks passed!"

# Documentation
doc:
	@echo "Generating documentation..."
	cargo doc --workspace --all-features --no-deps --open

# Infrastructure management
run-infra:
	@echo "Starting infrastructure..."
	docker-compose up -d postgres redis nats
	@echo "Waiting for services to be healthy..."
	@sleep 5
	docker-compose ps

stop-infra:
	@echo "Stopping infrastructure..."
	docker-compose down

restart-infra: stop-infra run-infra

logs:
	docker-compose logs -f

logs-postgres:
	docker-compose logs -f postgres

logs-nats:
	docker-compose logs -f nats

logs-redis:
	docker-compose logs -f redis

# Database operations
db-shell:
	psql postgres://hermes:hermes_dev@localhost:5432/hermes

db-reset:
	@echo "Resetting database..."
	docker-compose down -v postgres
	docker-compose up -d postgres
	@sleep 5
	@echo "Database reset complete"

# NATS operations
nats-sub:
	nats sub ">"

nats-sub-telemetry:
	nats sub "telemetry.>"

nats-sub-routes:
	nats sub "route.>"

nats-info:
	curl -s http://localhost:8222/varz | jq

# Monitoring
prometheus:
	@echo "Opening Prometheus..."
	@open http://localhost:9090 || xdg-open http://localhost:9090

grafana:
	@echo "Opening Grafana..."
	@open http://localhost:3000 || xdg-open http://localhost:3000

nats-monitor:
	@echo "Opening NATS Monitor..."
	@open http://localhost:8222 || xdg-open http://localhost:8222

# Development utilities
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -f flamegraph.svg
	rm -f perf.data*

coverage:
	@echo "Generating coverage report..."
	cargo tarpaulin --workspace --out Html
	@echo "Opening coverage report..."
	@open tarpaulin-report.html || xdg-open tarpaulin-report.html

watch:
	@echo "Running with auto-reload..."
	cargo watch -x "run --bin udp-ingestor"

# Install development tools
install-tools:
	@echo "Installing development tools..."
	cargo install cargo-watch
	cargo install cargo-tarpaulin
	cargo install flamegraph
	cargo install cargo-audit
	cargo install sqlx-cli --no-default-features --features postgres

# Security
audit:
	@echo "Running security audit..."
	cargo audit

# Quick start for new developers
setup: install-tools run-infra build
	@echo ""
	@echo "Setup complete! You can now run:"
	@echo "  make test      - Run tests"
	@echo "  make check     - Run all checks"
	@echo "  make help      - See all commands"
