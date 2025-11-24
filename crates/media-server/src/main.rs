use axum::{
    routing::{get, post, delete},
    Router,
    http::StatusCode,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use common::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://:redis_dev_password@localhost:6379".to_string());
    let nats_url = std::env::var("NATS_URL")
        .unwrap_or_else(|_| "nats://localhost:4222".to_string());

    let app_state = AppState::new("", &redis_url, &nats_url).await?;
    let state = Arc::new(app_state);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/sessions", post(create_session))
        .route("/sessions/:id", delete(close_session))
        .route("/sessions/:id/publish", post(publish_track))
        .route("/sessions/:id/subscribe", post(subscribe_track))
        .route("/sessions/:id/stats", get(get_stats))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8089".to_string()).parse::<u16>()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Media Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> StatusCode { StatusCode::OK }
async fn create_session() -> StatusCode { StatusCode::NOT_IMPLEMENTED }
async fn close_session() -> StatusCode { StatusCode::NOT_IMPLEMENTED }
async fn publish_track() -> StatusCode { StatusCode::NOT_IMPLEMENTED }
async fn subscribe_track() -> StatusCode { StatusCode::NOT_IMPLEMENTED }
async fn get_stats() -> StatusCode { StatusCode::NOT_IMPLEMENTED }
