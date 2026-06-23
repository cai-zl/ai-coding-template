use std::env;

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub host: String,
    pub port: u16,
    pub rust_log: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let _ = dotenvy::dotenv();

        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .context("DATABASE_URL is required")?,
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379/0".into()),
            host: env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("APP_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8080),
            rust_log: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info,backend=debug,sea_orm=warn".into()),
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
