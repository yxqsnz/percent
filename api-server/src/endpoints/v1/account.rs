use crate::{
    body::account_create::{AccountCreate, AccountJoin},
    database::{accounts::Accounts, Connection},
    utils::{token, RestResult},
};
use axum::{http::StatusCode, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use validator::Validate;

pub async fn post(
    Connection(mut db): Connection,
    Json(acc): Json<AccountCreate>,
) -> RestResult<StatusCode> {
    acc.validate()?;

    let password = bcrypt::hash(acc.password, 12)?;

    Accounts::create(
        &mut db,
        acc.nick,
        acc.full_name,
        password,
        token::generate(),
    )
    .await?;

    Ok(StatusCode::CREATED)
}

pub async fn get(
    Connection(mut db): Connection,
    jar: CookieJar,
    Json(details): Json<AccountJoin>,
) -> RestResult<(CookieJar, StatusCode)> {
    details.validate()?;

    let account = Accounts::find_by_nick(&mut db, details.nick).await.to()?;

    if bcrypt::verify(details.password, &account.password)? {
        let cookie = Cookie::new("Account.Token", account.token);

        Ok((jar.add(cookie), StatusCode::OK))
    } else {
        Ok((jar, StatusCode::UNAUTHORIZED))
    }
}
