# Hermes - Minimal Ride-Hailing Microservice System 🚕

Hermes is a **minimal ride-hailing platform** inspired by Uber. It is designed as a **microservices-based system** written in **Rust**, demonstrating clean architecture, event-driven communication, and distributed system design. The current version focuses on **core services only**, providing the foundation for future extensions.

---

## 🎯 Goals & Motivation

* Build a **production-like distributed system** using Rust.
* Practice **microservice communication** (via NATS, Redis, gRPC/REST).
* Learn and apply **design patterns** (Strategy, Observer, etc.) in a real-world simulation.
* Focus initially on **user management**, **location ingestion**, and **simulation-driven data flows**.

---

## 🏗️ Active Services

### 1. **Gateway**

* Unified HTTP API built with **Axum**.
* Acts as the API gateway for external requests.
* Forwards calls to internal services.

### 2. **Auth Service**

* Handles user registration and authentication (drivers/customers).
* Issues JWT tokens.

### 3. **Location Processor**

* Consumes location updates (from UDP Ingestor or Simulation).
* Processes driver movement history and estimated routes.
* Persists data in PostgreSQL and caches ephemeral data in Redis.

### 4. **UDP Ingestor**

* Listens for simulated location updates over **UDP**.
* Parses and publishes them to the message bus (NATS).

### 5. **Simulation Service**

* Generates **simulated vehicle and rider data**.
* Sends periodic location updates over UDP to test the system.

### 6. **Common Crate**

* Shared library with **models, DTOs, errors, and utilities**.
* Reduces code duplication across services.

---

## 🛠️ Tech Stack

* **Language:** Rust 🦀
* **Frameworks:**

  * [Axum](https://github.com/tokio-rs/axum) (HTTP)
  * [Tokio](https://tokio.rs/) (Async runtime)
* **Databases & Messaging:**

  * PostgreSQL (Persistent storage)
  * Redis (Cache & ephemeral state)
  * NATS (Event-driven messaging)
* **Utilities:**

  * `tracing` + `tracing-subscriber` (logging & observability)
  * `sqlx` (async DB queries)
  * `serde` (serialization)

---

## 📂 Project Structure

```
hermes/
 ├── crates/
 │   ├── common/              # Shared utilities, models, errors
 │   ├── udp-ingestor/        # UDP listener for location updates
 │   ├── location-processor/  # Location parsing & route builder
 │   ├── auth-service/        # Auth & user management
 │   ├── gateway/             # API gateway (Axum)
 │   ├── simulation/          # Data generator & sender
 ├── Cargo.toml
 └── README.md
```

---

📅 Roadmap 

We will follow a sprint-based development approach. Each sprint is 2 weeks long, starting from October 6, 2025, with the first sprint ending on October 19, 2025. The tasks have been reordered to first build the simulation service (data generator) and then develop the UDP Ingestor (listener).



🎯 Sprint 1 Goal: Establish foundation & data simulation (Oct 6 – Oct 19, 2025)
Setup project structure & workspace.
Implement common crate (models, errors, DTOs).
Develop simulation crate for test data (simulated drivers broadcasting locations).


🎯 Sprint 2 Goal: Location ingestion (Oct 20 – Nov 2, 2025)
Implement udp-ingestor service to receive data from simulation.
Validate integration between simulation → udp-ingestor.
Publish ingested data to message bus (NATS).


🎯 Sprint 3 Goal: Location processing (Nov 3 – Nov 16, 2025)
Implement location-processor service.
Parse location updates and calculate routes.
Store location history in PostgreSQL.
Cache latest driver states in Redis.


🎯 Sprint 4 Goal: Authentication & user management (Nov 17 – Nov 30, 2025)
Implement auth-service (user registration & login).
Add JWT-based authentication.
Integrate with PostgreSQL for user storage.


🎯 Sprint 5 Goal: Unified external API (Dec 1 – Dec 7, 2025)
Implement gateway (Axum-based API gateway).
Connect gateway routes to auth & location services.
Expose minimal external API for testing.

🎯 Sprint 6+ Goal: Integration & improvements (Dec 8, 2025 onward)
Perform end-to-end testing with simulation → UDP → processor → gateway.
Add observability (metrics, tracing, structured logs).
Optimize Redis/Postgres queries.
Prepare for future services (matcher, ride-service, notifications).

---

## 🚀 Running the Project

```bash
# Clone repo
git clone https://github.com/bulutcan99/hermes.git
cd hermes

# Run services
cargo run -p udp-ingestor
cargo run -p location-processor
cargo run -p auth-service
cargo run -p gateway
cargo run -p simulation
```

---

## 🔮 Future Improvements

* Add `matcher` service for nearest-driver assignment.
* Add `ride-service` for ride lifecycle management.
* Implement `notification-service` for push/email updates.
* Deploy on Kubernetes for scaling.

---

## 🤝 Contribution

Contributions, issues, and feature requests are welcome!
Please open an issue or PR if you want to improve Hermes.

---
