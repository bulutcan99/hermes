# Hermes: GPS Tracking & Processing System

A microservice-based GPS tracking and route processing system built with Rust,
focusing on real-time telemetry ingestion, route extraction, and analytics.

## 🎯 Project Goals

- **Real-time GPS telemetry ingestion** via UDP
- **Intelligent route extraction** from raw location data
- **Scalable microservice architecture** with event-driven communication
- **Production-ready observability** with metrics and tracing

## 🏗️ Architecture

```
Simulation → UDP:4000 → Ingestor → NATS → Location Processor → PostgreSQL
                                                ↓
                                            Gateway API
```

### Services

- **UDP Ingestor**: Receives raw telemetry via UDP, validates, publishes to NATS
- **Location Processor**: Converts telemetry into routes and segments using
  Strategy pattern
- **Gateway**: REST API for users, vehicles, and route queries
- **Simulation**: Test data generator for development

### Infrastructure

- **PostgreSQL 16 + PostGIS**: Geospatial data storage
- **NATS**: Event streaming and pub/sub
- **Redis**: Caching and real-time state
- **Prometheus + Grafana**: Metrics and monitoring

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.81+
- [Docker](https://www.docker.com/) & Docker Compose
- [NATS CLI](https://github.com/nats-io/natscli) (optional, for debugging)

### 1. Clone and Setup

```bash
git clone https://github.com/yourusername/hermes.git
cd hermes
```

### 2. Start Infrastructure

```bash
docker-compose up -d postgres redis nats
```

Wait for health checks to pass:

```bash
docker-compose ps
```

### 3. Build Workspace

```bash
cargo build --workspace
```

### 4. Run Tests

```bash
cargo test --workspace
```

## 📦 Project Structure

```
hermes/
├── Cargo.toml                 # Workspace configuration
├── docker-compose.yml         # Infrastructure services
├── crates/
│   ├── common/                # Shared utilities and event contracts
│   ├── udp-ingestor/         # UDP telemetry receiver
│   ├── location-processor/    # Route extraction service
│   ├── gateway/               # REST API service
│   ├── auth-service/          # Authentication module
│   └── simulation/            # Test data generator
├── infra/
│   ├── postgres/              # Database schemas
│   ├── prometheus/            # Metrics configuration
│   └── grafana/               # Dashboard configuration
└── docs/                      # Additional documentation
```

## 🔧 Development

### Run Services Locally

Each service can be run independently:

```bash
# Terminal 1: UDP Ingestor (Sprint 1)
cd crates/udp-ingestor
RUST_LOG=info cargo run

# Terminal 2: Simulator (Sprint 1)
cd crates/simulation
NUM_VEHICLES=5 cargo run

# Terminal 3: Location Processor (Sprint 2)
cd crates/location-processor
cargo run

# Terminal 4: Gateway (Sprint 3)
cd crates/gateway
cargo run
```

### Monitor NATS Events

```bash
# Subscribe to all telemetry events
nats sub "telemetry.>"

# Subscribe to route events
nats sub "route.>"
```

### Database Access

```bash
# Connect to PostgreSQL
psql postgres://hermes:hermes_dev@localhost:5432/hermes

# Example queries
SELECT * FROM vehicles;
SELECT * FROM routes WHERE status = 'active';
SELECT * FROM route_segments ORDER BY start_time DESC LIMIT 10;
```

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p common
cargo test -p udp-ingestor

# Run with output
cargo test --workspace -- --nocapture

# Run integration tests only
cargo test --workspace --test '*'
```

## 📊 Observability

### Prometheus Metrics

Access at: http://localhost:9090

Example queries:

- `up{job="nats"}` - NATS availability
- `go_memstats_alloc_bytes{job="nats"}` - NATS memory usage

### Grafana Dashboards

Access at: http://localhost:3000 (admin/admin)

Dashboards will be added in Sprint 4.

### NATS Monitoring

Access at: http://localhost:8222

- `/healthz` - Health check
- `/varz` - General information
- `/connz` - Connection information

## 🛠️ Configuration

All services use environment variables for configuration:

```bash
# Database
DATABASE_URL=postgres://hermes:hermes_dev@localhost:5432/hermes

# NATS
NATS_URL=nats://localhost:4222

# Redis
REDIS_URL=redis://localhost:6379

# Logging
RUST_LOG=info
```

## 📅 Development Roadmap

- [x] **Sprint 0**: Foundation (workspace, infra, CI)
- [ ] **Sprint 1**: UDP Ingestion Pipeline
- [ ] **Sprint 2**: Location Processor
- [ ] **Sprint 3**: Auth & Gateway API
- [ ] **Sprint 4**: Observability & Production Readiness

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Quality

```bash
# Format code
cargo fmt --all

# Run lints
cargo clippy --all-targets --all-features -- -D warnings

# Check documentation
cargo doc --workspace --all-features --no-deps
```

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for
details.

## 🙏 Acknowledgments

- Inspired by real-world ride-hailing telemetry systems
- Built with amazing Rust ecosystem tools
- PostgreSQL + PostGIS for geospatial capabilities

## 📚 Additional Resources

- [DEVELOPMENT.md](docs/DEVELOPMENT.md) - Detailed development guide
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - Architecture deep dive
- [API.md](docs/API.md) - API documentation
