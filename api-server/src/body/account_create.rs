use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AccountCreate {
    #[validate(length(min = 2, max = 32))]
    pub full_name: Option<String>,
    #[validate(length(min = 3, max = 16))]
    pub nick: String,
    #[validate(length(min = 8, max = 72))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct AccountJoin {
    #[validate(length(min = 3, max = 16))]
    pub nick: String,
    #[validate(length(min = 8, max = 72))]
    pub password: String,
}
