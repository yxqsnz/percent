use serde::Serialize;

#[derive(Serialize)]
pub enum Code {
    ValidationFailed,
    InternalServerError,
}
