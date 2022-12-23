pub mod error;
pub mod response;
pub mod token;

pub type RestResult<T> = Result<T, error::Error>;
