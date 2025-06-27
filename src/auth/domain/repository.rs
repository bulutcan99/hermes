use crate::auth::domain::error::AuthError;
use crate::auth::domain::model::{UserAuth, UserNewComer};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[async_trait]
pub trait AuthStorage: Send + Sync + 'static {
    async fn find_by_email(&self, email: String) -> Result<UserAuth, AuthError>;

    async fn create(&self, auth: UserAuth) -> Result<Uuid, AuthError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::data::hash;
    use async_trait::async_trait;
    use secrecy::ExposeSecret;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    pub struct MockAuthRepository {
        data: Arc<RwLock<HashMap<String, UserAuth>>>,
    }

    impl MockAuthRepository {
        pub fn new() -> Self {
            Self {
                data: Arc::new(RwLock::new(HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl AuthStorage for MockAuthRepository {
        async fn find_by_email(&self, email: String) -> Result<UserAuth, AuthError> {
            let data = self.data.read().await;
            data.get(&email).cloned().ok_or(AuthError::UserNotFound)
        }

        async fn create(&self, user_auth: UserAuth) -> Result<(), AuthError> {
            let mut data = self.data.write().await;
            data.insert(user_auth.email.clone(), user_auth);
            Ok(())
        }
    }

    #[tokio::test]
    async fn find_user_by_email_should_return_user_when_exists() {
        let repo = MockAuthRepository::new();

        let email = "test@example.com".to_string();
        let plain_password = "Password123!".to_string();
        let hashed_password = hash::hash_password(plain_password.as_str()).unwrap();

        let new_user = UserNewComer::new(email.clone(), hashed_password);
        let auth_user: UserAuth = new_user.into();

        repo.create(auth_user).await.unwrap();

        // Act
        let result = repo.find_by_email(email.clone()).await;

        // Assert
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, email);
        assert!(
            !user.password_hash.expose_secret().is_empty(),
            "Password hash should not be empty"
        );
    }

    #[tokio::test]
    async fn find_user_by_email_should_return_err_when_not_exists() {
        let repo = MockAuthRepository::new();

        let result = repo.find_by_email("notfound@example.com".to_string()).await;

        assert!(result.is_err());
    }
}
