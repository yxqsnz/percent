use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::error::DatabaseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Sqlx(#[from] sqlx::Error),

    #[error("{0:?}")]
    Bcrypt(#[from] bcrypt::BcryptError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        use Error::{Bcrypt, Sqlx};
        let (status, message) = match self {
            Sqlx(sqlx_error) => match sqlx_error {
                sqlx::Error::RowNotFound => {
                    tracing::error!("RowNotFound: {sqlx_error:?}");
                    (StatusCode::NOT_FOUND, "Item not found")
                }

                sqlx::Error::Database(err) if parse_db(err.as_ref()).is_some() => {
                    parse_db(err.as_ref()).unwrap()
                }

                error => {
                    tracing::error!(error = ?error, "Unknown error");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                }
            },

            Bcrypt(bc) => {
                tracing::error!(error = ?bc, "Bcrypt hash failed!");

                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

fn parse_db(error: &dyn DatabaseError) -> Option<(StatusCode, &'static str)> {
    let code = error.code()?;

    if *"23505" == code {
        Some((StatusCode::BAD_REQUEST, "Item already exists!"))
    } else {
        None
    }
}
