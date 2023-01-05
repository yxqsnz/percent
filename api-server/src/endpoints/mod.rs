mod v1;

use axum::{
    http::{HeaderValue, Method},
    Router,
};
use sqlx::{Pool, Postgres};
use tower_http::{
    cors::{AllowHeaders, CorsLayer},
    trace::TraceLayer,
};

macro_rules! endpoint {
    ($method:tt$(, $($other_method:tt),+)?: $($location:tt)+) => {{
        use $($location)+ as base;

        ::axum::routing::$method(base::$method)$(.$($other_method(base::$other_method))+)?
    }}
}

pub fn route(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/api/v1/account", endpoint!(post, get: v1::account))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers(AllowHeaders::any())
                .allow_methods([Method::GET]),
        )
        .with_state(pool)
}
