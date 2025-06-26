use crate::auth::domain::model::{UserAuth, UserNewComer};
use once_cell::sync::Lazy;
use regex::Regex;
use secrecy::SecretString;
use serde_derive::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_number = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric());

    if has_number && has_special {
        Ok(())
    } else {
        let mut err = ValidationError::new("invalid_password");
        err.message = Some(
            "Password must contain at least one number and one special character.".into(),
        );
        Err(err)
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserRegisterRequest {
    #[serde(default)]
    #[validate(email(message = "Email address is not valid."))]
    pub email: String,

    #[serde(default)]
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters."),
        custom(function = "validate_password")
    )]
    pub password: String,
}

impl From<UserRegisterRequest> for UserNewComer {
    fn from(request: UserRegisterRequest) -> Self {
        Self {
            email: request.email,
            password: SecretString::from(request.password),
        }
    }
}

#[derive(Serialize, Debug)]
struct UserRegisterResponse {
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_password() {
        let result = validate_password("Abcdef1!");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_missing_special_char() {
        let result = validate_password("Password1");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message.unwrap(),
            "Password must contain at least one number and one special character."
        );
    }

    #[tokio::test]
    async fn test_missing_number() {
        let result = validate_password("Password!");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_missing_both() {
        let result = validate_password("Password");
        assert!(result.is_err());
    }
}
