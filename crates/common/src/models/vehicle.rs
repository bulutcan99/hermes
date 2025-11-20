use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Vehicle {
    pub id: Uuid,
    pub driver_id: Uuid,
    pub license_plate: String,
    pub model: String,
    pub status: VehicleStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "vehicle_status", rename_all = "lowercase")]
pub enum VehicleStatus {
    Active,
    Inactive,
    Maintenance,
}
