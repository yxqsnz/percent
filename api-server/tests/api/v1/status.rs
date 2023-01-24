use crate::prepare;
use api_server::endpoints::route;
use axum::http::method::Method;
use hyper::{Body, Request, StatusCode};
use sqlx::PgPool;
use tower::ServiceExt;
use tracing_test::traced_test;
#[sqlx::test]
#[traced_test]

async fn status(pg: PgPool) {
    prepare(&pg).await;
    let app = route(pg);

    let req = Request::builder()
        .uri("/api/v1/status")
        .method(Method::GET)
        .body(Body::empty())
        .unwrap();

    let result = app.oneshot(req).await.unwrap();

    assert_eq!(result.status(), StatusCode::OK);
}
