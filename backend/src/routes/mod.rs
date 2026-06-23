pub mod health;
pub mod todo;

use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::state::SharedState;

pub fn router(state: SharedState) -> Router {
    let api = Router::new()
        .merge(health::routes())
        .merge(todo::routes());

    Router::new()
        .route("/healthz", axum::routing::get(health::healthz))
        .merge(api)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
