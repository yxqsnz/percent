pub mod macros;
pub mod token;

pub type RestResult<T> = Result<T, crate::response::error::Response>;
