use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found: {0}")]
    UserNotFound(String),
    #[error("Failed to read db: {0}")]
    DbReadError(String),
}
