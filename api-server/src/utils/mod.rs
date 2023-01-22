use crate::response::error::Error;

pub mod macros;
pub mod token;

pub type Result<T, E = Error> = std::result::Result<T, E>;
