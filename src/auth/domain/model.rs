use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use secrecy::SecretString;

#[derive(Debug, Clone)]
pub struct UserNewComer {
    pub email: String,
    pub password: SecretString,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserAuth {
    pub email: String,
    pub password_hash: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredentials {
    pub access_token: String,
    pub refresh_token: String,
}
