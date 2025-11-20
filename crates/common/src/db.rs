use std::path::PathBuf;
use crate::config::DatabaseConfig;
use crate::errors::{Error, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use sqlx::migrate::Migrator;
use tracing::info;

/// Create a PostgreSQL connection pool
pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&config.url)
        .await?;

    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let manifest_path = PathBuf::from(manifest_dir);
    let workspace_root = manifest_path
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| Error::Path("Path not found!".to_string()))?;
    let migrations_path = workspace_root.join("infra/postgres/migrations");

    info!("Running migrations on path: {}", migrations_path.display());
    let migrator = Migrator::new(migrations_path).await.map_err(|e| Error::Migration(e) )?;
    migrator.run(pool).await?;

    Ok(())
}

/// Health check for database connection
pub async fn health_check(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use testcontainers::{runners::AsyncRunner, ContainerAsync, ImageExt};
    use testcontainers_modules::postgres::Postgres;

    #[tokio::test]
    #[ignore] // Requires a running database instance
    async fn test_database_connection() {
        let config = DatabaseConfig {
            url: "postgresql://hermes:hermes_dev@localhost:5432/hermes".to_string(),
            max_connections: 5,
            min_connections: 1,
        };

        let pool = create_pool(&config).await;
        assert!(pool.is_ok());

        if let Ok(pool) = pool {
            let result = health_check(&pool).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_run_migrations_with_container() {
        // Start Postgres container
        let postgres_image = Postgres::default()
            .with_tag("16-alpine");

        let container = postgres_image
            .start()
            .await
            .expect("Failed to start postgres container");

        // Create Connection string
        let host = "127.0.0.1";
        let port = container
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get postgres port");

        let url = format!(
            "postgres://postgres:postgres@{}:{}/postgres",
            host, port
        );

        println!("🔗 Connecting to: {}", url);

        // Create Connection pool
        let pool = sqlx::PgPool::connect(&url)
            .await
            .expect("Failed to connect to postgres");

        println!("✅ Connected to database");

        // Run migrations
        let result = run_migrations(&pool).await;

        assert!(result.is_ok(), "Migrations should run successfully: {:?}", result);

        println!("✅ Migrations completed successfully");

        // Cleanup
        pool.close().await;
    }

    #[tokio::test]
    async fn test_migrations_are_idempotent() {
        // Test: Migrations should be runnable multiple times
        let postgres_image = Postgres::default()
            .with_tag("16-alpine");

        let container = postgres_image
            .start()
            .await
            .expect("Failed to start postgres container");

        let port = container
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get postgres port");

        let url = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            port
        );

        let pool = sqlx::PgPool::connect(&url)
            .await
            .expect("Failed to connect to postgres");

        // First migration run
        let result1 = run_migrations(&pool).await;
        assert!(result1.is_ok(), "First migration run failed");

        // Second migration run (should be idempotent)
        let result2 = run_migrations(&pool).await;
        assert!(result2.is_ok(), "Second migration run failed - not idempotent");

        pool.close().await;
    }

    #[tokio::test]
    async fn test_database_health_check_with_container() {
        let postgres_image = Postgres::default()
            .with_tag("16-alpine");

        let container = postgres_image
            .start()
            .await
            .expect("Failed to start postgres container");

        let port = container
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get postgres port");

        let url = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            port
        );

        let pool = sqlx::PgPool::connect(&url)
            .await
            .expect("Failed to connect to postgres");

        // Health check
        let result = health_check(&pool).await;
        assert!(result.is_ok(), "Health check should pass");

        pool.close().await;
    }

    #[tokio::test]
    async fn test_create_pool_with_config() {
        let postgres_image = Postgres::default()
            .with_tag("16-alpine");

        let container = postgres_image
            .start()
            .await
            .expect("Failed to start postgres container");

        let port = container
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get postgres port");

        let config = DatabaseConfig {
            url: format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port),
            max_connections: 10,
            min_connections: 2,
        };

        let pool = create_pool(&config).await;
        assert!(pool.is_ok(), "Pool creation should succeed");

        if let Ok(pool) = pool {
            // Test that the Pool is working
            let result = health_check(&pool).await;
            assert!(result.is_ok());

            pool.close().await;
        }
    }
}