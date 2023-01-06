use axum::response::IntoResponse;
use snafu::Snafu;
use validator::ValidationErrors;

use super::Structured;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    Validations { source: ValidationErrors },
    Sqlx { source: sqlx::Error },
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Structured::from(self).into_response()
    }
}
