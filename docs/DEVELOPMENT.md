# Development Guide

This guide provides detailed instructions for setting up your development
environment and contributing to Hermes.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Local Setup](#local-setup)
- [Running Services](#running-services)
- [Testing Strategy](#testing-strategy)
- [Code Style](#code-style)
- [Common Tasks](#common-tasks)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### Required

- **Rust 1.81+**: Install via [rustup](https://rustup.rs/)

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup component add rustfmt clippy
  ```

- **Docker & Docker Compose**:
  [Installation guide](https://docs.docker.com/get-docker/)

- **PostgreSQL Client** (psql): For database access

  ```bash
  # macOS
  brew install postgresql

  # Ubuntu/Debian
  sudo apt-get install postgresql-client

  # Windows
  # Download from https://www.postgresql.org/download/windows/
  ```

### Optional but Recommended

- **NATS CLI**: For debugging event streams

  ```bash
  # macOS
  brew install nats-io/nats-tools/nats

  # Linux/Windows
  # Download from https://github.com/nats-io/natscli/releases
  ```

- **HTTPie or curl**: For API testing

  ```bash
  brew install httpie  # macOS
  sudo apt install httpie  # Ubuntu
  ```

- **rust-analyzer**: For IDE support
  - VS Code: Install "rust-analyzer" extension
  - IntelliJ: Install "Rust" plugin

## Local Setup

### 1. Clone Repository

```bash
git clone https://github.com/yourusername/hermes.git
cd hermes
```

### 2. Environment Variables

Create `.env` file in project root:

```bash
# Database
DATABASE_URL=postgres://hermes:hermes_dev@localhost:5432/hermes

# NATS
NATS_URL=nats://localhost:4222

# Redis
REDIS_URL=redis://localhost:6379

# Logging
RUST_LOG=info,sqlx=warn

# JWT Secret (for auth service)
JWT_SECRET=your-secret-key-change-in-production

# Server ports
UDP_INGESTOR_PORT=4000
GATEWAY_PORT=8080
LOCATION_PROCESSOR_METRICS_PORT=9092
```

### 3. Start Infrastructure

```bash
# Start all infrastructure services
docker-compose up -d

# Verify services are running
docker-compose ps

# Check logs
docker-compose logs -f postgres
docker-compose logs -f nats
docker-compose logs -f redis
```

### 4. Initialize Database

The database schema is automatically initialized via `migrations.sql`, but you can
manually verify:

```bash
psql postgres://hermes:hermes_dev@localhost:5432/hermes

# Run some verification queries
\dt  -- List tables
\d routes  -- Describe routes table
SELECT * FROM vehicles;  -- Check sample data
```

### 5. Build Workspace

```bash
# Build all crates in debug mode
cargo build --workspace

# Build in release mode
cargo build --workspace --release

# Build specific crate
cargo build -p common
cargo build -p udp-ingestor
```

## Running Services

### Method 1: Individual Services (Development)

Each service runs independently for easier debugging:

```bash
# Terminal 1: UDP Ingestor
cd crates/udp-ingestor
RUST_LOG=debug cargo run

# Terminal 2: Location Processor (Sprint 2+)
cd crates/location-processor
cargo run

# Terminal 3: Gateway (Sprint 3+)
cd crates/gateway
cargo run

# Terminal 4: Simulation
cd crates/simulation
NUM_VEHICLES=5 UPDATE_INTERVAL_MS=2000 cargo run
```

### Method 2: Watch Mode (Auto-reload)

Install `cargo-watch`:

```bash
cargo install cargo-watch
```

Run with auto-reload:

```bash
cd crates/udp-ingestor
cargo watch -x run
```

### Method 3: Docker (Full Stack)

Coming in Sprint 4 - will build and run all services in containers.

## Testing Strategy

### Unit Tests

Test individual functions and modules:

```bash
# Run all unit tests
cargo test --workspace --lib

# Run tests for specific crate
cargo test -p common
cargo test -p udp-ingestor

# Run specific test
cargo test haversine_distance

# Run with output
cargo test -- --nocapture --test-threads=1
```

### Integration Tests

Test service interactions:

```bash
# Run integration tests (requires infrastructure)
docker-compose up -d
cargo test --workspace --test '*'
```

### Test Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --workspace --out Html

# Open report
open tarpaulin-report.html
```

### Load Testing (Sprint 4)

```bash
# Install k6
brew install k6  # macOS

# Run load test
k6 run tests/load/telemetry_ingestion.js
```

## Code Style

### Formatting

```bash
# Check formatting
cargo fmt --all -- --check

# Auto-format
cargo fmt --all
```

### Linting

```bash
# Run clippy with strict rules
cargo clippy --all-targets --all-features -- -D warnings

# Fix auto-fixable issues
cargo clippy --all-targets --all-features --fix
```

### Code Conventions

1. **Error Handling**: Use `Result<T, Error>`, avoid `.unwrap()` in production
   code
2. **Naming**:
   - Functions: `snake_case`
   - Types: `PascalCase`
   - Constants: `SCREAMING_SNAKE_CASE`
3. **Documentation**: All public items must have doc comments
4. **Imports**: Group by std → external → internal
5. **Async**: Prefer `async/await` over manual futures

Example:

```rust
use std::collections::HashMap;

use tokio::net::UdpSocket;
use tracing::{error, info};

use crate::domain::Route;
use common::events::TelemetryEvent;

/// Process incoming telemetry event
///
/// # Arguments
/// * `event` - The telemetry event to process
///
/// # Returns
/// Processed route if successful
///
/// # Errors
/// Returns error if validation fails or database operation fails
pub async fn process_telemetry(
    event: &TelemetryEvent,
) -> Result<Route, ProcessingError> {
    // Implementation
}
```

## Common Tasks

### Add New Dependency

```bash
# Add to specific crate
cd crates/udp-ingestor
cargo add tokio --features full

# Add to workspace (in root Cargo.toml [workspace.dependencies])
# Then reference in crate with: tokio = { workspace = true }
```

### Create New Service

```bash
# Create new crate
cargo new --lib crates/my-service

# Add to workspace members in root Cargo.toml
# [workspace]
# members = [
#     "crates/common",
#     "crates/my-service",  # Add this
# ]
```

### Database Migrations

```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Create migration
sqlx migrate add create_my_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### Debug NATS Events

```bash
# Subscribe to all events
nats sub ">"

# Subscribe to telemetry events
nats sub "telemetry.*"

# Publish test event
nats pub telemetry.test '{"vehicle_id":"test","timestamp":"2024-01-01T00:00:00Z","lat":41.0,"lon":29.0}'

# Check subjects
nats sub -echo
```

### Access Logs

```bash
# Infrastructure logs
docker-compose logs -f postgres
docker-compose logs -f nats
docker-compose logs -f redis

# Service logs (when running locally)
RUST_LOG=debug cargo run  # Detailed logs
RUST_LOG=trace cargo run  # Very detailed
```

### Profile Performance

```bash
# Install flamegraph
cargo install flamegraph

# Run with profiling
sudo cargo flamegraph --bin udp-ingestor

# Opens flamegraph.svg in browser
```

## Troubleshooting

### Issue: Port Already in Use

```bash
# Find process using port
lsof -i :4000  # UDP Ingestor
lsof -i :5432  # PostgreSQL
lsof -i :4222  # NATS

# Kill process
kill -9 <PID>
```

### Issue: Database Connection Failed

```bash
# Check if PostgreSQL is running
docker-compose ps postgres

# Check logs
docker-compose logs postgres

# Restart PostgreSQL
docker-compose restart postgres

# Connect manually to verify
psql postgres://hermes:hermes_dev@localhost:5432/hermes
```

### Issue: NATS Connection Failed

```bash
# Check NATS status
curl http://localhost:8222/varz

# Check connections
curl http://localhost:8222/connz

# Restart NATS
docker-compose restart nats
```

### Issue: Cargo Build Fails

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for dependency conflicts
cargo tree
```

### Issue: Tests Failing

```bash
# Ensure infrastructure is running
docker-compose up -d

# Wait for health checks
sleep 10

# Run tests with output
cargo test -- --nocapture

# Run single test
cargo test test_name -- --exact --nocapture
```

### Issue: Out of Disk Space

```bash
# Clean Docker volumes
docker-compose down -v

# Clean cargo cache
cargo clean
rm -rf ~/.cargo/registry
rm -rf ~/.cargo/git

# Clean Docker images
docker system prune -a
```

## Git Workflow

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `refactor/description` - Code refactoring
- `docs/description` - Documentation updates

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add route segment detection algorithm
fix: handle null coordinates in telemetry
docs: update API documentation
refactor: extract validation logic to separate module
test: add integration tests for location processor
chore: update dependencies
```

### Pull Request Checklist

Before submitting PR:

- [ ] `cargo fmt --all` runs without changes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] New code has tests (>80% coverage)
- [ ] Documentation updated if needed
- [ ] CHANGELOG.md updated
- [ ] No `println!()` or `dbg!()` in code (use `tracing`)

## IDE Configuration

### VS Code

Recommended extensions:

- rust-analyzer
- Better TOML
- Error Lens
- Docker

`.vscode/settings.json`:

```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

### IntelliJ IDEA

1. Install Rust plugin
2. Settings → Languages & Frameworks → Rust:
   - Enable "Use rustfmt"
   - Enable "Run Clippy on save"
3. Add run configurations for each service

## Performance Tips

1. **Use release builds for benchmarking**:

   ```bash
   cargo build --release
   ./target/release/udp-ingestor
   ```

2. **Profile before optimizing**:

   ```bash
   cargo flamegraph --bin udp-ingestor
   ```

3. **Check dependencies compile time**:

   ```bash
   cargo build --timings
   # Opens cargo-timing.html
   ```

4. **Use workspace inheritance** to reduce duplicate dependencies

5. **Enable LTO for release**: Already configured in root `Cargo.toml`:
   ```toml
   [profile.release]
   lto = true
   ```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [NATS Documentation](https://docs.nats.io/)
- [PostGIS Documentation](https://postgis.net/documentation/)
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)
