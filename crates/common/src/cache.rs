use redis::{Client, AsyncCommands};
use crate::error::{AppError, Result};

pub struct CacheClient {
    client: Client,
}

impl CacheClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.client.get_async_connection()
            .await
            .map_err(|e| AppError::Cache(e.to_string()))?;
        
        conn.get(key).await.map_err(Into::into)
    }

    pub async fn set(&self, key: &str, value: &str, ttl: Option<usize>) -> Result<()> {
        let mut conn = self.client.get_async_connection()
            .await
            .map_err(|e| AppError::Cache(e.to_string()))?;
        
        if let Some(ttl) = ttl {
            conn.set_ex(key, value, ttl).await.map_err(Into::into)
        } else {
            conn.set(key, value).await.map_err(Into::into)
        }
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.client.get_async_connection()
            .await
            .map_err(|e| AppError::Cache(e.to_string()))?;
        
        conn.del(key).await.map_err(Into::into)
    }
}
