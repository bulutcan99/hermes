//! Configuration utilities for loading environment variables

use std::env;
use std::str::FromStr;

/// Load environment variable with default value
///
/// # Example
/// ```
/// use common::config::env_var_or;
///
/// let host = env_var_or("HOST", "localhost");
/// ```
pub fn env_var_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Parse environment variable to type T with default
///
/// # Example
/// ```
/// use common::config::env_var_parse;
///
/// let port: u16 = env_var_parse("PORT", 8080);
/// let timeout: i64 = env_var_parse("TIMEOUT", 300);
/// ```
pub fn env_var_parse<T>(key: &str, default: T) -> T
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    env::var(key)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(default)
}

/// Get database URL from environment
pub fn database_url() -> String {
    env_var_or(
        "DATABASE_URL",
        "postgres://hermes:hermes_dev@localhost:5432/hermes",
    )
}

/// Get NATS URL from environment
pub fn nats_url() -> String {
    env_var_or("NATS_URL", "nats://localhost:4222")
}

/// Get Redis URL from environment
pub fn redis_url() -> String {
    env_var_or("REDIS_URL", "redis://localhost:6379")
}

/// Get log level from environment
pub fn log_level() -> String {
    env_var_or("RUST_LOG", "info")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_var_or() {
        // Should return default when var not set
        let value = env_var_or("NONEXISTENT_VAR_12345", "default");
        assert_eq!(value, "default");
    }

    #[test]
    fn test_env_var_parse() {
        // Should return default when var not set
        let value: u16 = env_var_parse("NONEXISTENT_PORT_12345", 8080);
        assert_eq!(value, 8080);

        // Should parse valid values
        env::set_var("TEST_PORT", "3000");
        let value: u16 = env_var_parse("TEST_PORT", 8080);
        assert_eq!(value, 3000);
        env::remove_var("TEST_PORT");
    }

    #[test]
    fn test_default_urls() {
        // These should always return valid URLs
        let db_url = database_url();
        assert!(db_url.starts_with("postgres://"));

        let nats_url = nats_url();
        assert!(nats_url.starts_with("nats://"));

        let redis_url = redis_url();
        assert!(redis_url.starts_with("redis://"));
    }
}
