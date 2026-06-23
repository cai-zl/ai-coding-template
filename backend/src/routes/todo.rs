use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use redis::AsyncCommands;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::entity::Todo;
use crate::entity::todo::ActiveModel as TodoActiveModel;
use crate::state::SharedState;

const CACHE_TTL: u64 = 300;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/api/todos", get(list).post(create))
        .route("/api/todos/:id", get(get_one).put(update).delete(remove))
}

async fn list(State(state): State<SharedState>) -> impl IntoResponse {
    match Todo::find().all(&state.db).await {
        Ok(todos) => (
            StatusCode::OK,
            Json(serde_json::to_value(todos).unwrap_or_default()),
        )
            .into_response(),
        Err(e) => {
            tracing::error!(error = ?e, "list todos failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal"})),
            )
                .into_response()
        }
    }
}

async fn get_one(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let cache_key = format!("todo:{id}");
    let mut conn = state.redis.clone();
    if let Ok(Some(cached)) = conn.get::<_, Option<String>>(&cache_key).await {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&cached) {
            return (StatusCode::OK, Json(value)).into_response();
        }
    }

    match Todo::find_by_id(id).one(&state.db).await {
        Ok(Some(todo)) => {
            if let Ok(payload) = serde_json::to_string(&todo) {
                let _: redis::RedisResult<()> =
                    conn.set_ex(&cache_key, payload, CACHE_TTL).await;
            }
            (
                StatusCode::OK,
                Json(serde_json::to_value(todo).unwrap_or_default()),
            )
                .into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "not found"})),
        )
            .into_response(),
        Err(e) => {
            tracing::error!(error = ?e, "get todo failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal"})),
            )
                .into_response()
        }
    }
}

#[derive(serde::Deserialize)]
struct CreatePayload {
    title: String,
    #[serde(default)]
    done: bool,
}

async fn create(
    State(state): State<SharedState>,
    Json(payload): Json<CreatePayload>,
) -> impl IntoResponse {
    let model = TodoActiveModel {
        title: Set(payload.title),
        done: Set(payload.done),
        created_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    match model.insert(&state.db).await {
        Ok(todo) => (
            StatusCode::CREATED,
            Json(serde_json::to_value(todo).unwrap_or_default()),
        )
            .into_response(),
        Err(e) => {
            tracing::error!(error = ?e, "create todo failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal"})),
            )
                .into_response()
        }
    }
}

#[derive(serde::Deserialize)]
struct UpdatePayload {
    title: Option<String>,
    done: Option<bool>,
}

async fn update(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePayload>,
) -> impl IntoResponse {
    let existing = match Todo::find_by_id(id).one(&state.db).await {
        Ok(Some(m)) => m,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "not found"})),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!(error = ?e, "find todo failed");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal"})),
            )
                .into_response();
        }
    };

    let mut active: TodoActiveModel = existing.into();
    if let Some(title) = payload.title {
        active.title = Set(title);
    }
    if let Some(done) = payload.done {
        active.done = Set(done);
    }

    match active.update(&state.db).await {
        Ok(todo) => {
            let cache_key = format!("todo:{id}");
            let _: redis::RedisResult<()> = state.redis.clone().del(&cache_key).await;
            (
                StatusCode::OK,
                Json(serde_json::to_value(todo).unwrap_or_default()),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!(error = ?e, "update todo failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal"})),
            )
                .into_response()
        }
    }
}

async fn remove(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match Todo::delete_by_id(id).exec(&state.db).await {
        Ok(r) if r.rows_affected > 0 => {
            let cache_key = format!("todo:{id}");
            let _: redis::RedisResult<()> = state.redis.clone().del(&cache_key).await;
            (
                StatusCode::NO_CONTENT,
                Json(serde_json::Value::Null),
            )
                .into_response()
        }
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "not found"})),
        )
            .into_response(),
        Err(e) => {
            tracing::error!(error = ?e, "delete todo failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal"})),
            )
                .into_response()
        }
    }
}
