//! Json serializable response for axum

use axum::{
    response::{IntoResponse, Response as AxumResponse},
    Json,
};
use serde::Serialize;

use super::{raw::Raw, Code};

/// JSON Serializable response
#[derive(Serialize)]
pub struct Response {
    pub code: u32,
    pub error: super::Code,
    pub detail: Vec<String>,
}

impl IntoResponse for Response {
    fn into_response(self) -> AxumResponse {
        Json(self).into_response()
    }
}

impl Response {
    pub fn from_raw(raw_error: Raw, code: Option<Code>, messages: Vec<String>) -> Self {
        match raw_error {
            Raw::Validation(val) => Self {
                code: 422,
                error: Code::ValidationFailed,
                detail: {
                    if let Some(dt) = val.message {
                        messages.push(dt.to_string());
                    }
                    messages
                },
            },

            Raw::Sqlx(e) => Self {
                code: 500,
                error: code.unwrap_or(Code::InternalServerError),
                detail: Vec::with_capacity(0),
            },
        }
    }
}
