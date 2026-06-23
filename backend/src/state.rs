use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use redis::aio::ConnectionManager;
use redis::Client as RedisClient;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use tokio::time::sleep;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: ConnectionManager,
}

impl AppState {
    pub async fn connect(database_url: &str, redis_url: &str) -> Result<Self> {
        let db = connect_db_with_retry(database_url, 5).await?;
        let redis = connect_redis_with_retry(redis_url, 5).await?;
        Ok(Self { db, redis })
    }
}

async fn connect_db_with_retry(url: &str, attempts: u32) -> Result<DatabaseConnection> {
    let mut last_err = None;
    for i in 0..attempts {
        match Database::connect(url).await {
            Ok(conn) => return Ok(conn),
            Err(e) => {
                last_err = Some(e);
                sleep(Duration::from_secs(2u64.pow(i))).await;
            }
        }
    }
    Err(anyhow::anyhow!(
        "Failed to connect to database after {attempts} attempts: {last_err:?}"
    ))
}

async fn connect_redis_with_retry(url: &str, attempts: u32) -> Result<ConnectionManager> {
    let client = RedisClient::open(url).context("invalid REDIS_URL")?;
    let mut last_err = None;
    for i in 0..attempts {
        match ConnectionManager::new(client.clone()).await {
            Ok(conn) => return Ok(conn),
            Err(e) => {
                last_err = Some(e);
                sleep(Duration::from_secs(2u64.pow(i))).await;
            }
        }
    }
    Err(anyhow::anyhow!(
        "Failed to connect to redis after {attempts} attempts: {last_err:?}"
    ))
}

pub type SharedState = Arc<AppState>;
