use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use axum::http::{HeaderValue, Method, header};

mod cache;
mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod routes;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env
    dotenvy::dotenv().ok();

    // Init tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
        )
        .init();

    // Load config
    let config = config::Config::from_env()?;
    tracing::info!("Starting server in {} mode", config.app_env);

    // Connect database
    let db = db::create_pool(&config.database_url, config.database_max_connections).await?;

    // Run migrations
    db::run_migrations(&db).await?;

    // Connect Redis
    let redis = cache::create_redis(&config.redis_url).await?;

    // Build app state
    let state = AppState {
        db,
        redis,
        config: Arc::new(config.clone()),
    };

    // CORS setup
    let cors_origins: Vec<HeaderValue> = config
        .cors_origins
        .iter()
        .filter_map(|o| o.parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(cors_origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .allow_credentials(true);

    // Build router
    let app = routes::create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Start server
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server berjalan di http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
