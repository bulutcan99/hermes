use tracing::info;
use crate::common::config::environment::Environment;
use crate::common::logger;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Environment::from_env()
        .load()
        .expect("Environment loading failed!");
    logger::init();
    info!("Logger initialized");
    Ok(())
}

