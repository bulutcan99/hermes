use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {

    #[error("Hash error: {0}")]
    HashError(#[from] argon2::Error),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User ID not found in auth table")]
    UserNotFound,

    #[error("Email already exists")]
    EmailExists,

    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
}