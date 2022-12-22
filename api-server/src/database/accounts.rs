use sqlx::{pool::PoolConnection, Postgres};

pub struct Accounts;

impl Accounts {
    pub async fn create(
        db: &mut PoolConnection<Postgres>,
        nick: String,
        full_name: Option<String>,
        password: String,
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO accounts(nick, name, password) VALUES ($1, $2, $3)")
            .bind(nick)
            .bind(full_name)
            .bind(password)
            .execute(db)
            .await?;

        Ok(())
    }
}
