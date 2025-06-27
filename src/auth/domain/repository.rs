use crate::auth::domain::error::AuthError;
use crate::auth::domain::model::{UserAuth, UserNewComer};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[async_trait]
pub trait AuthStorage: Send + Sync + 'static {
    async fn find_by_email(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        email: String,
    ) -> Result<UserAuth, AuthError>;

    async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_auth: UserAuth,
    ) -> Result<Uuid, AuthError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use secrecy::{ExposeSecret, SecretString};
    use sqlx::Transaction;
    use sqlx::postgres::Postgres;
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
        async fn find_by_email(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            email: String,
        ) -> Result<UserAuth, AuthError> {
            let data = self.data.read().await;
            data.get(&email).cloned().ok_or(AuthError::UserNotFound)
        }

        async fn create(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            user_auth: UserAuth,
        ) -> Result<(), AuthError> {
            let mut data = self.data.write().await;
            data.insert(user_auth.email.clone(), user_auth);
            Ok(())
        }
    }

    fn dummy_tx() -> &'static mut Transaction<'static, Postgres> {
        // test ortamında asla kullanılmayacak, ama interface sağlamak için var
        unsafe {
            static mut DUMMY: Option<Transaction<'static, Postgres>> = None;
            &mut *(std::ptr::null_mut())
        }
    }

    #[tokio::test]
    async fn find_user_by_email_should_return_user_when_exists() {
        let repo = MockAuthRepository::new();

        let email = "test@example.com".to_string();
        let plain_password = "Password123!".to_string();

        let new_user = UserNewComer::new(email.clone(), plain_password);
        let hashed_user = new_user.hash_password().unwrap();
        let auth_user: UserAuth = hashed_user.into();
        let mut tx = dummy_tx();

        repo.create(&mut tx, auth_user.clone()).await.unwrap();

        // Act
        let result = repo.find_by_email(&mut tx, email.clone()).await;

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
        let mut tx = dummy_tx();

        let result = repo
            .find_by_email(&mut tx, "notfound@example.com".to_string())
            .await;

        assert!(result.is_err());
    }
}
