use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Account {
    pub nick: String,
    pub name: Option<String>,
    pub password: String,
    pub token: String,
    pub created_at: OffsetDateTime,
}
