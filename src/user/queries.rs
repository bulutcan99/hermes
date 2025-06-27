use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use crate::auth::domain::error::AuthError;
use crate::auth::domain::model::{UserAuth, UserNewComer};
use crate::auth::domain::repository::DatabaseAuthRepository;

pub struct AuthStorage{
    db: Arc<Pool<Postgres>>,
}

impl AuthStorage{
    pub fn new(db: Arc<Pool<Postgres>>) -> AuthStorage{
        Self { db}
    }
}

#[async_trait]
impl DatabaseAuthRepository for AuthStorage {
    async fn find_by_email(&self, email: String) -> Result<UserAuth, AuthError> {
        let db = self.db.clone();
        let row = sqlx::query!(
            r#"
            SELECT id, email, password_hash, refresh_token, created_at, updated_at
            FROM "auth"
            WHERE email = $1
            "#,
            email
        )
            .fetch_optional(&*db)
            .await?;


    }
    async fn create(&self, user_auth: UserNewComer) -> Result<(), AuthError> {
        let db = self.db.clone();
        let result = sqlx::query!(
            r#"
            INSERT INTO "auth" (email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            "#,
            user_auth.email,
            user_auth.password.as_string(),
            user_auth.created_at.convert_to_offset(),
            user_auth.updated_at.convert_to_offset(),
        )
            .fetch_one(&*db)
            .await?;

       Ok(())
    }
}