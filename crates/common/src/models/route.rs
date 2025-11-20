use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Route {
    pub id: Uuid,
    pub vehicle_id: Uuid,
    pub status: RouteStatus,
    pub start_location: Option<sqlx::types::JsonValue>,
    pub end_location: Option<sqlx::types::JsonValue>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub total_distance: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "route_status", rename_all = "lowercase")]
pub enum RouteStatus {
    Active,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RouteSegment {
    pub id: Uuid,
    pub route_id: Uuid,
    pub vehicle_id: Uuid,
    pub start_location: sqlx::types::JsonValue,
    pub end_location: sqlx::types::JsonValue,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub distance: f64,
    pub avg_speed: f64,
    pub created_at: DateTime<Utc>,
}