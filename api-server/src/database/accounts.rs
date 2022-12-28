use sqlx::{pool::PoolConnection, Postgres};
use time::OffsetDateTime;

use crate::models::account::Account;
pub struct Accounts;
type RawAccount = (String, Option<String>, String, String, OffsetDateTime);

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
        let (nick, name, password, account_token, created_at) =
            sqlx::query_as::<_, RawAccount>("SELECT * FROM accounts where token=$1")
                .bind(token)
                .fetch_one(db)
                .await?;

        Ok(Account {
            nick,
            name,
            password,
            token: account_token,
            created_at,
        })
    }

    pub async fn find_by_nick(
        db: &mut PoolConnection<Postgres>,
        nick: String,
    ) -> sqlx::Result<Account> {
        let (nick, name, password, token, created_at) =
            sqlx::query_as::<_, RawAccount>("SELECT * FROM accounts where nick=$1")
                .bind(nick)
                .fetch_one(db)
                .await?;

        Ok(Account {
            nick,
            name,
            password,
            token,
            created_at,
        })
    }
}
