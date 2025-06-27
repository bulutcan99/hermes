use crate::auth::domain::error::AuthError;
use crate::auth::domain::model::{UserAuth, UserNewComer};
use crate::auth::domain::repository::AuthStorage;
use async_trait::async_trait;
use secrecy::ExposeSecret;
use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Default)]
pub struct DatabaseAuthRepository;

#[async_trait]
impl AuthStorage for DatabaseAuthRepository {
    async fn find_by_email(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        email: String,
    ) -> Result<UserAuth, AuthError> {
        let row = sqlx::query_as!(
            UserAuth,
            r#"
            SELECT id, email, password_hash, refresh_token, created_at, updated_at
            FROM auth
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(tx)
        .await?;

        Ok(row)
    }

    async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_auth: UserAuth,
    ) -> Result<Uuid, AuthError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO auth (pid, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING pid
            "#,
            user_auth.id,
            user_auth.email,
            user_auth.password_hash.expose_secret(),
            user_auth.created_at.convert_to_offset(),
            user_auth.updated_at.convert_to_offset()
        )
        .fetch_one(tx)
        .await?;

        Ok(result.pid)
    }
}
