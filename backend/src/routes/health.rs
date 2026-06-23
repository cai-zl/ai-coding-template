use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use crate::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new().route("/api/health", get(health))
}

pub async fn healthz() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

pub async fn health(State(state): State<SharedState>) -> impl IntoResponse {
    let db_ok = state.db.ping().await.is_ok();
    let mut redis = state.redis.clone();
    let redis_ok = redis::cmd("PING").query_async::<String>(&mut redis).await.is_ok();

    let status = if db_ok && redis_ok { "ok" } else { "degraded" };
    let code = if db_ok && redis_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        code,
        Json(json!({
            "service": "api",
            "status": status,
            "checks": {
                "db": if db_ok { "ok" } else { "fail" },
                "redis": if redis_ok { "ok" } else { "fail" },
            }
        })),
    )
}
