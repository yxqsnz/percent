use axum::{http::StatusCode, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    body::account_create::{AccountCreate, AccountJoin},
    database::{accounts::Accounts, Connection},
    utils::{token, RestResult},
};

pub async fn post(
    Connection(mut db): Connection,
    Json(acc): Json<AccountCreate>,
) -> RestResult<StatusCode> {
    let password = bcrypt::hash(acc.password.0, 8)?;

    Accounts::create(&mut db, acc.nick.0, acc.full_name.map(|v| v.0), password).await?;

    Ok(StatusCode::CREATED)
}

// login
pub async fn get(
    Connection(mut db): Connection,
    jar: CookieJar,
    Json(details): Json<AccountJoin>,
) -> RestResult<(CookieJar, StatusCode)> {
    let account = Accounts::find_by_nick(&mut db, details.nick.0).await?;

    if bcrypt::verify(details.password.0, &account.password)? {
        let cookie = Cookie::new("Account.Token", token::generate_from_account(&account));

        Ok((jar.add(cookie), StatusCode::OK))
    } else {
        Ok((jar, StatusCode::UNAUTHORIZED))
    }
}
