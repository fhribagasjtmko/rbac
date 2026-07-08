use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::errors::AppResult;

pub async fn create_redis(redis_url: &str) -> anyhow::Result<Arc<Mutex<MultiplexedConnection>>> {
    let client = Client::open(redis_url)?;
    let conn = client.get_multiplexed_async_connection().await?;
    tracing::info!("Redis connected");
    Ok(Arc::new(Mutex::new(conn)))
}

// ─── Key builders ────────────────────────────────────────────────────────────

pub fn refresh_token_key(user_id: &str) -> String {
    format!("refresh_token:{}", user_id)
}

pub fn blacklist_key(jti: &str) -> String {
    format!("blacklist:{}", jti)
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

pub async fn set_ex(
    conn: &Arc<Mutex<MultiplexedConnection>>,
    key: &str,
    value: &str,
    ttl_seconds: i64,
) -> AppResult<()> {
    let mut c = conn.lock().await;
    c.set_ex::<_, _, ()>(key, value, ttl_seconds as u64).await?;
    Ok(())
}

pub async fn get(
    conn: &Arc<Mutex<MultiplexedConnection>>,
    key: &str,
) -> AppResult<Option<String>> {
    let mut c = conn.lock().await;
    let val: Option<String> = c.get(key).await?;
    Ok(val)
}

pub async fn del(
    conn: &Arc<Mutex<MultiplexedConnection>>,
    key: &str,
) -> AppResult<()> {
    let mut c = conn.lock().await;
    c.del::<_, ()>(key).await?;
    Ok(())
}

pub async fn exists(
    conn: &Arc<Mutex<MultiplexedConnection>>,
    key: &str,
) -> AppResult<bool> {
    let mut c = conn.lock().await;
    let result: bool = c.exists(key).await?;
    Ok(result)
}
