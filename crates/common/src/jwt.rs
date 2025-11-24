use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use crate::{models::JwtClaims, error::{AppError, Result}};
use uuid::Uuid;

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate_token(&self, user_id: Uuid, username: &str, expiry: i64) -> Result<String> {
        let now = chrono::Utc::now().timestamp();
        let claims = JwtClaims {
            sub: user_id,
            username: username.to_string(),
            exp: now + expiry,
            iat: now,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Jwt(e.to_string()))
    }

    pub fn verify_token(&self, token: &str) -> Result<JwtClaims> {
        let token_data = decode::<JwtClaims>(
            token,
            &self.decoding_key,
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| AppError::Jwt(e.to_string()))?;

        Ok(token_data.claims)
    }
}
