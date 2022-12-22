pub mod error;
pub mod response;
pub mod validators;

pub type RestResult<T> = Result<T, error::Error>;
