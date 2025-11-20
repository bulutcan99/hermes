//! Error types for Hermes services

use thiserror::Error;

/// Common result type with our Error
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for Hermes services
#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("NATS error: {0}")]
    Nats(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Path error: {0}")]
    Path(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    /// Returns HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            Error::Database(_) => 500,
            Error::Migration(_) => 500,
            Error::Redis(_) => 500,
            Error::Nats(_) => 500,
            Error::Serialization(_) => 400,
            Error::Authentication(_) => 401,
            Error::Authorization(_) => 403,
            Error::Validation(_) => 400,
            Error::NotFound(_) => 404,
            Error::Conflict(_) => 409,
            Error::Internal(_) => 500,
            Error::Config(_) => 500,
            Error::Path(_) => 500,
            Error::Other(_) => 500,
        }
    }

    /// Returns whether this error should be logged as an error vs warning
    pub fn is_server_error(&self) -> bool {
        self.status_code() >= 500
    }
}

// Convert async-nats errors
impl From<async_nats::Error> for Error {
    fn from(err: async_nats::Error) -> Self {
        Error::Nats(err.to_string())
    }
}

impl From<async_nats::PublishError> for Error {
    fn from(err: async_nats::PublishError) -> Self {
        Error::Nats(err.to_string())
    }
}

impl From<async_nats::SubscribeError> for Error {
    fn from(err: async_nats::SubscribeError) -> Self {
        Error::Nats(err.to_string())
    }
}
