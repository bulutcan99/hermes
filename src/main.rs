use crate::common::config::db::DB;
use crate::common::config::environment::Environment;
use tracing::info;
use crate::common::config::logger;

mod common;
mod auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Environment::from_env()
        .load()
        .expect("Environment loading failed!");
    logger::init();
    info!("Logger initialized");
    let db = DB::new().await?;
    info!("DB initialized!");
    Ok(())
}
