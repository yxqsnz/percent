use axum::{
    http::{HeaderValue, Method},
    Router,
};
use sqlx::{Pool, Postgres};
use tower_http::cors::{AllowHeaders, CorsLayer};

pub fn route(pool: Pool<Postgres>) -> Router {
    Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers(AllowHeaders::any())
                .allow_methods([Method::GET]),
        )
        .with_state(pool)
}
