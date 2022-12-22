use serde::Deserialize;

use crate::utils::validators::MaxSizedString;

#[derive(Deserialize)]
pub struct AccountCreate {
    pub full_name: Option<MaxSizedString<32>>,
    pub nick: MaxSizedString<16>,
    pub password: MaxSizedString<72>,
}
