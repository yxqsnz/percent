use axum::{response::IntoResponse, Json};
use serde::Serialize;

use super::Error;

#[derive(Serialize, Debug)]
pub enum Code {
    ValidationFailed,
    InternalServerError,
}

#[derive(Serialize, Debug)]
pub struct Structured {
    error: Code,
    messages: Vec<String>,
}

impl IntoResponse for Structured {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl From<Error> for Structured {
    fn from(value: Error) -> Self {
        match value {
            Error::Validations { source } => Self {
                error: Code::ValidationFailed,
                messages: source
                    .errors()
                    .into_iter()
                    .map(|(a, b)| format!("{a}: {b:?}"))
                    .collect(),
            },
            Error::Sqlx { source } => {
                tracing::error!("A sqlx error has happended: {source}");
                Self {
                    error: Code::InternalServerError,
                    messages: vec!["Internal server error".to_string()],
                }
            }
        }
    }
}
