use once_cell::sync::Lazy;
use regex::Regex;
use secrecy::SecretString;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use crate::auth::domain::model::{UserAuth, UserNewComer};

pub static PASSWORD_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?=.*[0-9])(?=.*[^A-Za-z0-9])").unwrap());
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserRegisterRequest {
    #[serde(default)]
    #[validate(email(message = "Email address is not valid."))]
    pub email: String,

    #[serde(default)]
    #[validate(length(min = 8, message = "Password must be at least 8 characters."))]
    #[validate(regex(
        path = "PASSWORD_REGEX",
        message = "Password must contain at least one digit and one special character."
    ))]
    pub password: String,
}

impl From<UserRegisterRequest> for UserNewComer{
    fn from(request: UserRegisterRequest) -> Self {
        Self{
            email: request.email,
            password:  SecretString::from(request.password)

        }
    }
}

#[derive(Serialize, Debug)]
struct UserRegisterResponse {
    message: String,
}

