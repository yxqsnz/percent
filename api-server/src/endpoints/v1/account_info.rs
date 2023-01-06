use crate::{database::accounts::Accounts, models::account::Account, utils::error::Miracle};
use axum::Json;
use axum_extra::extract::CookieJar;

use crate::{database::Connection, utils::RestResult};

pub async fn get(Connection(mut db): Connection, jar: CookieJar) -> RestResult<Json<Account>> {
    let token = jar.get("Account.Token").unwrap_or_default().value();

    let account = Accounts::find_by_token(&mut db, token).await?;

    Ok(Json(account))
}
