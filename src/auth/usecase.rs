use crate::auth::domain::error::AuthError;
use crate::auth::domain::model::{AuthCredentials, Plain, UserAuth, UserNewComer};
use crate::auth::domain::repository::AuthStorage;
use crate::auth::domain::service::AuthService;
use crate::common::data::hash;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthUseCase<K>
where
    K: AuthStorage,
{
    pool: PgPool,
    repo: Arc<K>,
}
impl<K> AuthUseCase<K>
where
    K: AuthStorage,
{
    pub fn new(pool: PgPool, repo: Arc<K>) -> AuthUseCase<K> {
        AuthUseCase { pool, repo }
    }
}

#[async_trait]
impl<K> AuthService for AuthUseCase<K>
where
    K: AuthStorage,
{
    async fn login_user(&self, user_auth: UserAuth) -> Result<AuthCredentials, AuthError> {
        todo!()
    }

    async fn register_user(&self, user_new: UserNewComer<Plain>) -> Result<(), AuthError> {
        let hashed_user = user_new.hash_password()?;
        let mut tx = self.pool.begin().await?;
        let user_auth: UserAuth = hashed_user.into();

        match self.repo.create(user_auth).await {
            Ok(()) => {
                tx.commit().await?;
                Ok(())
            }
            Err(err) => {
                tracing::error!("Error creating user auth: {err}");
                tx.rollback().await?;
                Err(err)
            }
        }

    }
}
