use api_server::endpoints::route;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
    Router,
};
use serde_json::json;
use sqlx::PgPool;
use tower::{Service, ServiceExt};
use tracing_test::traced_test;

use crate::{json_body, prepare};

async fn register(app: &mut Router) {
    let req = Request::builder()
        .uri("/api/v1/account")
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(json_body! {
            "nick": "ypeople",
            "password": "12345678"
        })
        .unwrap();

    let res = app.ready().await.unwrap().call(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
}

#[sqlx::test]
#[traced_test]
async fn register_account(db: PgPool) {
    prepare(&db).await;

    let mut app = route(db);
    register(&mut app).await;
}

#[sqlx::test]
#[traced_test]
async fn register_and_login(db: PgPool) {
    prepare(&db).await;
    let mut app = route(db);
    register(&mut app).await;

    let req = Request::builder()
        .uri("/api/v1/account")
        .method(http::Method::GET)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(json_body! {
            "nick": "ypeople",
            "password": "12345678"
        })
        .unwrap();

    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}
