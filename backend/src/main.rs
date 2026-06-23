mod config;
mod entity;
mod migration;
mod routes;
mod state;

use std::sync::Arc;

use anyhow::Result;
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::migration::SchemaMigrator;
use crate::routes::router;
use crate::state::AppState;
use sea_orm_migration::MigratorTrait;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_new(&config.rust_log).unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let state = AppState::connect(&config.database_url, &config.redis_url).await?;
    SchemaMigrator::up(&state.db, None).await?;
    tracing::info!("database migrations applied");

    let shared = Arc::new(state);
    let app = router(shared);

    let listener = tokio::net::TcpListener::bind(config.bind_addr()).await?;
    tracing::info!("listening on {}", config.bind_addr());
    axum::serve(listener, app).await?;
    Ok(())
}
