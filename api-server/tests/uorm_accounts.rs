use api_server::database::accounts::Accounts;
use sqlx::PgPool;

#[sqlx::test]
async fn create_account(db: PgPool) {
    sqlx::migrate!("../migrations")
        .run(&db)
        .await
        .expect("Migration failed");

    let res = Accounts::create(
        &mut db.acquire().await.expect("Can't adquire pool"),
        "test_xd".to_string(),
        Some("John Y.".to_string()),
        "12345678".to_string(),
    )
    .await;

    eprintln!("{res:?}");
    assert!(res.is_ok());
}

#[sqlx::test]
async fn create_and_find_account(db: PgPool) {
    sqlx::migrate!("../migrations")
        .run(&db)
        .await
        .expect("Migration failed");

    let mut db = db.acquire().await.unwrap();

    let res = Accounts::create(
        &mut db,
        "test_xd".to_string(),
        Some("John Y.".to_string()),
        "12345678".to_string(),
    )
    .await;

    eprintln!("{res:?}");
    assert!(res.is_ok());

    let account = Accounts::find_by_nick(&mut db, "test_xd".to_string())
        .await
        .unwrap();

    assert_eq!(account.name, Some("John Y.".to_string()));
}
