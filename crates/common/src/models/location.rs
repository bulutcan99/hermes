use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    pub vehicle_id: Uuid,
    pub location: Location,
    pub speed: f64,
    pub heading: f64,
    pub timestamp: DateTime<Utc>,
}