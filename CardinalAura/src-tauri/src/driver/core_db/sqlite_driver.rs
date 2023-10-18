use sqlx::{
  sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
  SqlitePool,
};

pub async fn initialize_connection() -> Result<SqlitePool, sqlx::Error>{
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
        SqliteConnectOptions::new()
            .filename("core_db.sqlite")
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal),
        )
        .await
        .unwrap();

    Ok(pool)
}

pub async fn migrate_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
  sqlx::migrate!("./src/db/initial_setup")
    .run(pool)
    .await?;

    Ok(())
}