use api_server::{endpoints::route, models::account::Account};
use axum::{
    body::Body,
    http::{
        self,
        header::{COOKIE, SET_COOKIE},
        Request, StatusCode,
    },
    Router,
};
use hyper::body::to_bytes;
use serde_json::json;
use sqlx::PgPool;
use tower::{Service, ServiceExt};
use tracing_test::traced_test;

use crate::{json_body, prepare};

fn account_details() -> Body {
    json_body! {
        "nick": "baldi",
        "password": "12345678"
    }
}

async fn register(app: &mut Router) {
    let req = Request::builder()
        .uri("/api/v1/account/create")
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(account_details())
        .unwrap();

    let res = app.ready().await.unwrap().call(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
}

async fn login(app: &mut Router) -> Option<String> {
    let req = Request::builder()
        .uri("/api/v1/account/login")
        .method(http::Method::POST)
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(account_details())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    res.headers()
        .get(SET_COOKIE)
        .and_then(|h| h.to_str().ok())
        .map(|i| i.split_once("="))
        .flatten()
        .map(|i| i.1)
        .map(ToString::to_string)
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
    login(&mut app).await.unwrap();
}

#[sqlx::test]
#[traced_test]
async fn account_info(db: PgPool) {
    prepare(&db).await;
    let mut app = route(db);
    register(&mut app).await;
    let token = login(&mut app).await.unwrap();

    let req = Request::builder()
        .uri("/api/v1/account/info")
        .method(http::Method::GET)
        .header(COOKIE, format!("Account.Token={token}"))
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), 200);

    let body: Account = to_bytes(res.into_body())
        .await
        .map(|x| serde_json::from_slice(&x))
        .unwrap()
        .unwrap();

    assert_eq!(body.nick, "baldi");
}
