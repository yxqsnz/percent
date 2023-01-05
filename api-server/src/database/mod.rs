use crate::response::error::Because;
pub mod accounts;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use sqlx::{pool::PoolConnection, PgPool, Postgres};

use crate::response::error::Response;

pub struct Connection(pub PoolConnection<Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for Connection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool
            .acquire()
            .await
            .because(None, Some(["Failed to connect to database!".to_string()]))?;

        Ok(Self(conn))
    }
}
