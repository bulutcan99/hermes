//! Event contracts for inter-service communication via NATS
//!
//! These are the ONLY shared data structures between services.
//! Domain models are service-specific.

pub mod telemetry;

pub use telemetry::{RouteCompletedEvent, RouteSegmentEvent, TelemetryEvent};
