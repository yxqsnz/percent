use axum::Json;

use crate::{
    body::account_create::AccountCreate,
    database::{accounts::Accounts, Connection},
    utils::{response::internal_error, RestResult},
};

pub async fn post(
    Connection(mut db): Connection,
    Json(acc): Json<AccountCreate>,
) -> RestResult<()> {
    let password = bcrypt::hash(acc.password.0, 8).map_err(internal_error)?;

    Accounts::create(&mut db, acc.nick.0, acc.full_name.map(|v| v.0), password)
        .await
        .map_err(internal_error)?;

    Ok(())
}
