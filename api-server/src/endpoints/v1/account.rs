use crate::{
    body::account_create::{AccountCreate, AccountJoin},
    database::{accounts::Accounts, Connection},
    response::{BcryptSnafu, SqlxSnafu, ValidationsSnafu},
    utils::{token, Result},
};
use axum::{http::StatusCode, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use snafu::ResultExt;
use validator::Validate;

pub async fn post(
    Connection(mut db): Connection,
    Json(data): Json<AccountCreate>,
) -> Result<StatusCode> {
    data.validate().context(ValidationsSnafu)?;

    let password = bcrypt::hash(data.password, 12).context(BcryptSnafu)?;

    Accounts::create(
        &mut db,
        data.nick,
        data.full_name,
        password,
        token::generate(),
    )
    .await
    .context(SqlxSnafu)?;

    Ok(StatusCode::CREATED)
}

pub async fn get(
    Connection(mut db): Connection,
    jar: CookieJar,
    Json(data): Json<AccountJoin>,
) -> Result<(CookieJar, StatusCode)> {
    data.validate().context(ValidationsSnafu)?;
    let account = Accounts::find_by_nick(&mut db, data.nick)
        .await
        .context(SqlxSnafu)?;

    if bcrypt::verify(data.password, &account.password).context(BcryptSnafu)? {
        let cookie = Cookie::new("Account.Token", account.token);

        Ok((jar.add(cookie), StatusCode::OK))
    } else {
        Ok((jar, StatusCode::UNAUTHORIZED))
    }
}
