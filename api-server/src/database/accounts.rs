use sqlx::{pool::PoolConnection, Postgres};
use time::{Date, Duration, OffsetDateTime};

use crate::models::account::Account;

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

    pub async fn find_by_nick(
        db: &mut PoolConnection<Postgres>,
        nick: String,
    ) -> sqlx::Result<Account> {
        let (nick, name, password, created_at) = sqlx::query_as::<
            _,
            (String, Option<String>, String, OffsetDateTime),
        >("SELECT * FROM accounts where nick=$1")
        .bind(nick)
        .fetch_one(db)
        .await?;

        Ok(Account {
            nick,
            name,
            password,
            created_at,
        })
    }
}
