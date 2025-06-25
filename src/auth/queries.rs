use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use crate::auth::domain::error::UserAuthError;
use crate::auth::domain::model::{UserAuth, UserNewComer};
use crate::auth::domain::repository::UserAuthRepository;

pub struct UserAuthRepo{
    db: Arc<Pool<Postgres>>,
}

#[async_trait]
impl UserAuthRepository for UserAuthRepo {
    async fn find_by_email(&self, email: String) -> Result<Option<UserAuth>, UserAuthError> {
        todo!()
    }

    async fn create(&self, user_auth: UserNewComer) -> Result<(), UserAuthError> {
        todo!()
    }
}