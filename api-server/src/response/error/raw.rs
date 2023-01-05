use parse_display::Display;
use thiserror::Error;
use validator::ValidationError;

#[derive(Error, Debug, Display)]
pub enum Raw {
    #[display("{0}")]
    Validation(#[from] ValidationError),

    #[display("{0}")]
    Sqlx(#[from] sqlx::Error),
}
