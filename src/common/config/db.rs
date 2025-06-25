use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::warn;
use crate::common::config::config::Config;

#[derive(Clone)]
pub struct DB {
    pub pool: Arc<PgPool>,
}

impl DB {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let config = Config::get();
        let mut attempts = 0;
        let pool = loop {
            attempts += 1;
            match PgPoolOptions::new()
                .max_connections(config.database.max_connections)
                .connect(&config.database.uri)
                .await {
                Ok(pool) => break Arc::new(pool),
                Err(err) => {
                    if attempts >= 3 {
                        return Err(err);
                    }

                    warn!("Postgres not ready yet ({:?}), retrying in 1s… (attempt {}/{})",err, attempts, 3);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        };

        Ok(DB {
            pool,
        })
    }
}
