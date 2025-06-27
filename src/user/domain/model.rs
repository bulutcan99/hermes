use crate::common::data::hash;
use secrecy::SecretString;
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

// Kullanici ilk olarak kayit olurken
#[derive(Debug, Clone)]
pub struct User {}

impl User {
    pub fn new(email: String, password: String) -> Self {
        todo!()
    }
}
