use sqlx::PgPool;
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: Arc<Mutex<MultiplexedConnection>>,
    pub config: Arc<Config>,
}
