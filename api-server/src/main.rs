mod body;
mod database;
mod endpoints;
mod utils;

use anyhow::Result;
use axum::Server;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env::var, net::SocketAddr, str::FromStr};
use tracing::info;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Connecting to database");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&var("Database.URL")?)
        .await?;

    sqlx::migrate!("../migrations").run(&db).await?;
    let addr = SocketAddr::from_str(&var("Api.ServeURL")?)?;
    info!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(endpoints::route(db).into_make_service())
        .await?;

    Ok(())
}
