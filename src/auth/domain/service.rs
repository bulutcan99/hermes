use crate::auth::domain::error::AuthError;
use crate::auth::domain::model::{AuthCredentials, Plain, UserAuth, UserNewComer};
use async_trait::async_trait;

#[async_trait]
pub trait AuthService {
    async fn login_user(&self, user_auth: UserAuth) -> Result<AuthCredentials, AuthError>;
    async fn register_user(&self, user_new: UserNewComer<Plain>) -> Result<(), AuthError>;
}
