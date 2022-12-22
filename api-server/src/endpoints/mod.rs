mod v1;

use axum::{
    http::{HeaderValue, Method},
    Router,
};
use sqlx::{Pool, Postgres};
use tower_http::cors::{AllowHeaders, CorsLayer};

macro_rules! endpoint {
    ($method:tt: $($location:tt)+) => {
        ::axum::routing::$method($($location)+::$method)
    }
}

pub fn route(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/api/v1/status", endpoint!(get: v1::status))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers(AllowHeaders::any())
                .allow_methods([Method::GET]),
        )
        .with_state(pool)
}
