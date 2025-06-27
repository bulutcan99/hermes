use async_trait::async_trait;
use sqlx::Transaction;
use crate::auth::domain::error::AuthError;
use crate::auth::domain::model::{UserAuth, UserNewComer};
use crate::common::config::db::DB;
use crate::user::domain::error::UserError;
use crate::user::domain::model::User;

#[async_trait]
pub trait DatabaseUserRepository {
    async fn create(&self, tx: &mut Transaction<'_, DB>, user: User) -> Result<User, UserError>;
}
