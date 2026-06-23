use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use tower::ServiceExt;

async fn handler() -> &'static str {
    "ok"
}

#[tokio::test]
async fn smoke_router_builds() {
    let app = axum::Router::new().route("/healthz", get(handler));

    let resp = app
        .oneshot(Request::builder().uri("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let _ = Arc::new(());
}
