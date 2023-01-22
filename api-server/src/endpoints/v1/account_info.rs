use crate::response::{MissingValueSnafu, SqlxSnafu};
use crate::{database::accounts::Accounts, models::account::Account};
use axum::Json;
use axum_extra::extract::CookieJar;
use snafu::{OptionExt, ResultExt};

use crate::{database::Connection, utils::Result};

pub async fn get(Connection(mut db): Connection, jar: CookieJar) -> Result<Json<Account>> {
    let token = jar.get("Account.Token").context(MissingValueSnafu {
        text: String::from("missing Account.Token cookie"),
    })?;

    let account = Accounts::find_by_token(&mut db, token.value())
        .await
        .context(SqlxSnafu)?;

    Ok(Json(account))
}
