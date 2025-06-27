use std::marker::PhantomData;
use crate::auth::domain::error::AuthError;
use crate::common::data::date::Timestamp;
use crate::common::data::hash;
use secrecy::{ExposeSecret, SecretString};
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::encode::IsNull::No;
use uuid::Uuid;

pub struct Plain;
pub struct Hashed;

#[derive(Debug, Clone)]
pub struct UserNewComer<T> {
    pub email: String,
    pub password: SecretString,
    _marker: PhantomData<T>,
}
impl UserNewComer<Plain> {
    pub fn new(email: String, plain_password: String) -> Self {
        Self {
            email,
            password: SecretString::from(plain_password.as_str()),
            _marker: PhantomData,
        }
    }

    pub fn hash_password(self) -> Result<UserNewComer<Hashed>, AuthError> {
        let hashed = hash::hash_password(&self.password.expose_secret())?;
        Ok(UserNewComer {
            email: self.email,
            password: SecretString::from(hashed),
            _marker: PhantomData,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserAuth {
    pub id: Uuid,
    pub email: String,
    pub password_hash: SecretString,
    pub refresh_token: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl From<UserNewComer<Hashed>> for UserAuth {
    fn from(user: UserNewComer<Hashed>) -> Self {
        Self{
            id:Uuid::new_v4(),
            email,
            password_hash: user.password,
            refresh_token:None,
            created_at: Timestamp::now_utc(),
            updated_at: Timestamp::now_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    pub access_token: String,
    pub refresh_token: String,
}
