use sqlx::{pool::PoolConnection, Postgres};

use crate::models::account::Account;
pub struct Accounts;

impl Accounts {
    pub async fn create(
        db: &mut PoolConnection<Postgres>,
        nick: String,
        full_name: Option<String>,
        password: String,
        token: String,
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO accounts(nick, name, password, token) VALUES ($1, $2, $3, $4)")
            .bind(nick)
            .bind(full_name)
            .bind(password)
            .bind(token)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn find_by_token(
        db: &mut PoolConnection<Postgres>,
        token: &str,
    ) -> sqlx::Result<Account> {
        sqlx::query_as::<_, Account>("SELECT * FROM accounts where token=$1")
            .bind(token)
            .fetch_one(db)
            .await
    }

    pub async fn find_by_nick(
        db: &mut PoolConnection<Postgres>,
        nick: String,
    ) -> sqlx::Result<Account> {
        sqlx::query_as::<_, Account>("SELECT * FROM accounts where nick=$1")
            .bind(nick)
            .fetch_one(db)
            .await
    }
}
