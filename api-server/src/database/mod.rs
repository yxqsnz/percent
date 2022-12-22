pub mod accounts;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use sqlx::{pool::PoolConnection, PgPool, Postgres};

use crate::utils::response::internal_error;

pub struct Connection(pub PoolConnection<Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for Connection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}
