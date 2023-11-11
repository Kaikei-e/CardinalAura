use sqlx::{Pool, sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous}, Sqlite, SqlitePool};
use std::str::FromStr;

pub struct SqliteDriver {
    pub pool: SqlitePool,
}

impl SqliteDriver {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        SqliteDriver { pool }
    }
}

pub async fn initialize_connection(db_url: String) -> Result<SqlitePool, sqlx::Error> {
    println!("Database file: {:?}", db_url);

    let pool = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .filename(db_url)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(pool)
        .await?;

    Ok(pool)
}

pub async fn migrate_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./db/initial_setup/")
        .run(pool)
        .await
        .expect("failed to migrate db");

    Ok(())
}
