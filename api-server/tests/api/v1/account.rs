use api_server::endpoints::route;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use tracing_test::traced_test;

#[sqlx::test]
#[traced_test]
async fn register_account(db: PgPool) {
    sqlx::migrate!("../migrations")
        .run(&db)
        .await
        .expect("Migration failed");

    let app = route(db);
    let req = Request::builder()
        .uri("/api/v1/account")
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(
            json! ({
                "nick": "test",
                "password": "ayo12345678"
            })
            .to_string(),
        ))
        .unwrap();

    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
}
