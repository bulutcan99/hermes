use async_trait::async_trait;
use crate::auth::domain::error::UserAuthError;
use crate::auth::domain::model::{UserAuth, UserCredentials, UserNewComer};

#[async_trait]
pub trait UserAuthService {
    async fn login_user(&self, user_auth: UserAuth) -> Result<Option<UserCredentials>, UserAuthError>;
    async fn create_user(&self, user_auth: UserNewComer) -> Result<(), UserAuthError>;
}