use time::OffsetDateTime;

pub struct Account {
    pub nick: String,
    pub name: Option<String>,
    pub password: String,
    pub created_at: OffsetDateTime,
}
