use async_trait::async_trait;
use crate::auth::domain::error::AuthError;
use crate::auth::domain::model::{UserAuth, AuthCredentials, UserNewComer};
use crate::user::domain::model::User;

#[async_trait]
pub trait UserAuthService {
    async fn register_user(&self, user_auth: User) -> Result<(), AuthError>;
}