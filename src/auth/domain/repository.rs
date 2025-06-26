use async_trait::async_trait;
use crate::auth::domain::error::UserAuthError;
use crate::auth::domain::model::{UserAuth, UserNewComer};


#[async_trait]
pub trait DatabaseAuthRepository {
    /// Finds a user by email.
    ///
    /// # Arguments
    ///
    /// * `email` - The email address of the user to search for.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(UserAuth))` if a user is found.
    /// * `Ok(None)` if no user is found.
    /// * `Err(UserAuthError)` if an error occurs (e.g., database error).
    async fn find_by_email(&self, email: String) -> Result<Option<UserAuth>, UserAuthError>;

    /// Creates a new user authentication record.
    ///
    /// # Arguments
    ///
    /// * `user_auth` - The data of the new user to create.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the user is created successfully.
    /// * `Err(UserAuthError)` if an error occurs.
    async fn create(&self, user_auth: UserNewComer) -> Result<(), UserAuthError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tokio::sync::RwLock;
    use std::sync::Arc;
    use secrecy::SecretString;

    /// A simple in-memory implementation of `DatabaseAuthRepository` for testing purposes.
    struct MockUserAuthRepository {
        data: Arc<RwLock<HashMap<String, UserAuth>>>,
    }

    #[async_trait]
    impl DatabaseAuthRepository for MockUserAuthRepository {
        async fn find_by_email(&self, email: String) -> Result<Option<UserAuth>, UserAuthError> {
            let data = self.data.read().await;
            Ok(data.get(&email).cloned())
        }

        async fn create(&self, user_auth: UserNewComer) -> Result<(), UserAuthError> {
            let mut data = self.data.write().await;
            data.insert(user_auth.email.clone(), UserAuth {
                email: user_auth.email,
                password_hash: format!("hashed-{:?}", user_auth.password),
                refresh_token: None,
            });
            Ok(())
        }
    }

    /// ✅ Test that verifies a user can be found by email after being created.
    #[tokio::test]
    async fn find_user_by_email_should_return_user_when_exists() {
        // Arrange
        let repo = MockUserAuthRepository {
            data: Arc::new(RwLock::new(HashMap::new())),
        };

        let new_user = UserNewComer {
            email: "test@example.com".to_string(),
            password: SecretString::from("password123".to_string()),
        };

        println!("NEW USER {:?}", new_user.password);

        repo.create(new_user.clone()).await.unwrap();

        // Act
        let result = repo.find_by_email(new_user.email.clone()).await.unwrap();

        assert!(result.is_some());
        let user = result.unwrap();
        assert_eq!(user.email, new_user.email);
        // Secrets can't be debug!
        assert_eq!(user.password_hash, "hashed-SecretBox<str>([REDACTED])");
    }

    /// ✅ Test that verifies `find_by_email` returns `None` when the user does not exist.
    #[tokio::test]
    async fn find_user_by_email_should_return_none_when_not_exists() {
        // Arrange
        let repo = MockUserAuthRepository {
            data: Arc::new(RwLock::new(HashMap::new())),
        };

        // Act
        let result = repo.find_by_email("notfound@example.com".to_string()).await.unwrap();

        // Assert
        assert!(result.is_none());
    }
}