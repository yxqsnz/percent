use axum::response::IntoResponse;
use bcrypt::BcryptError;
use snafu::Snafu;
use validator::{ValidationError, ValidationErrors};

use super::Structured;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    Validations { source: ValidationErrors },
    Validation { source: ValidationError },
    Bcrypt { source: BcryptError },
    Sqlx { source: sqlx::Error },
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Structured::from(self).into_response()
    }
}
