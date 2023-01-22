pub mod accounts;

use crate::response::error::SqlxSnafu;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use snafu::ResultExt;
use sqlx::{pool::PoolConnection, PgPool, Postgres};

use crate::response::error::Error;

pub struct Connection(pub PoolConnection<Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for Connection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.context(SqlxSnafu)?;

        Ok(Self(conn))
    }
}
