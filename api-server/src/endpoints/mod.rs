mod v1;

use axum::{
    http::{HeaderValue, Method},
    routing, Router,
};
use sqlx::{Pool, Postgres};
use tower_http::{
    cors::{AllowHeaders, CorsLayer},
    trace::TraceLayer,
};

pub fn route(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/api/v1/status", routing::get(v1::status::get))
        .route("/api/v1/account/info", routing::get(v1::account::info))
        .route("/api/v1/account/create", routing::post(v1::account::create))
        .route("/api/v1/account/login", routing::post(v1::account::login))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers(AllowHeaders::any())
                .allow_methods([Method::GET]),
        )
        .with_state(pool)
}
