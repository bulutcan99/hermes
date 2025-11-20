//! Configuration helpers for loading from environment variables

use crate::errors::{Error, Result};
use std::env;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            url: get_env("DATABASE_URL")?,
            max_connections: get_env_or("DATABASE_MAX_CONNECTIONS", "10")
                .parse()
                .map_err(|e| Error::Config(format!("Invalid DATABASE_MAX_CONNECTIONS: {}", e)))?,
            min_connections: get_env_or("DATABASE_MIN_CONNECTIONS", "2")
                .parse()
                .map_err(|e| Error::Config(format!("Invalid DATABASE_MIN_CONNECTIONS: {}", e)))?,
        })
    }
}

/// Redis configuration
#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
}

impl RedisConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            url: get_env("REDIS_URL")?,
        })
    }
}

/// NATS configuration
#[derive(Debug, Clone)]
pub struct NatsConfig {
    pub url: String,
}

impl NatsConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            url: get_env("NATS_URL")?,
        })
    }
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            host: get_env_or("SERVER_HOST", "0.0.0.0"),
            port: get_env_or("SERVER_PORT", "8080")
                .parse()
                .map_err(|e| Error::Config(format!("Invalid SERVER_PORT: {}", e)))?,
        })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Observability configuration
#[derive(Debug, Clone)]
pub struct ObservabilityConfig {
    pub log_level: String,
    pub metrics_port: u16,
}

impl ObservabilityConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            log_level: get_env_or("RUST_LOG", "info"),
            metrics_port: get_env_or("METRICS_PORT", "9090")
                .parse()
                .map_err(|e| Error::Config(format!("Invalid METRICS_PORT: {}", e)))?,
        })
    }
}

/// JWT configuration
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            secret: get_env("JWT_SECRET")?,
            expiration_hours: get_env_or("JWT_EXPIRATION_HOURS", "24")
                .parse()
                .map_err(|e| Error::Config(format!("Invalid JWT_EXPIRATION_HOURS: {}", e)))?,
        })
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get required environment variable
fn get_env(key: &str) -> Result<String> {
    env::var(key).map_err(|_| Error::Config(format!("Missing environment variable: {}", key)))
}

/// Get optional environment variable with default
fn get_env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Initialize tracing subscriber from environment
pub fn init_tracing() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_address() {
        let config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        assert_eq!(config.address(), "127.0.0.1:8080");
    }
}
