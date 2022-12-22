use axum::http::StatusCode;

pub mod response;
pub mod validators;

pub type RestResult<T> = Result<T, (StatusCode, String)>;
