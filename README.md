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

## 📅 Roadmap (Simplified)

**Week 1-2**

* Setup project structure & workspace.
* Implement `common` crate (models, errors, DTOs).
* Build `udp-ingestor` → receives & publishes location updates.

**Week 3-4**

* Implement `location-processor` with PostgreSQL/Redis.
* Add `auth-service` for user registration/login.

**Week 5**

* Develop `simulation` crate for test data.
* Ensure UDP ingestion works with simulated drivers.

**Week 6**

* Create `gateway` for unified external API.
* Integrate services through NATS & Redis.

**Week 7+**

* Perform end-to-end testing with ride data simulation.
* Add observability (metrics, tracing).
* Optimize system for scaling.

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
