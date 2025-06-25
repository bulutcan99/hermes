use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserAuthError {
    #[error("Failed to read db: {0}")]
    DbReadError(String),
}
