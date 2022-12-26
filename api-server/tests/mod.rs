use sqlx::PgPool;

mod api;
mod database;

async fn prepare(db: &PgPool) {
    sqlx::migrate!("../migrations")
        .run(db)
        .await
        .expect("Migration failed");
}

#[macro_export]
macro_rules! json_body {
    ($($tt:tt)*) => {
        Body::from(json!({$($tt)*}).to_string())
    };
}
