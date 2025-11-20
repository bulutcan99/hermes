use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::route::RouteStatus;
use crate::models::user::UserRole;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub role: UserRole,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
    pub role: UserRole,
}

#[derive(Debug, Deserialize)]
pub struct CreateVehicleRequest {
    pub driver_id: Uuid,
    pub license_plate: String,
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct RouteQuery {
    pub vehicle_id: Option<Uuid>,
    pub status: Option<RouteStatus>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}
