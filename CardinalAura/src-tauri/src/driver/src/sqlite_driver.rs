use anyhow::Error;
use futures::executor::block_on;
use port::repository::repository_port::{DbConnection, RepositoryPort};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Pool, Sqlite, SqlitePool,
};
use std::str::FromStr;

pub struct SqliteConnectionContext {
    pool: Pool<Sqlite>,
}

impl RepositoryPort for SqliteConnectionContext {
    fn new() -> Self {
        const DATABASE_DIR: &str = "driver/db/data";
        const DATABASE_FILE: &str = "core_db.sqlite";

        let base_dir = std::env::current_dir().expect("Can't access the current directory");

        let app_dir = base_dir.join("src");

        let database_dir = app_dir.join(DATABASE_DIR);
        let database_file = database_dir.join(DATABASE_FILE);

        let is_db_exist = std::fs::metadata(database_dir.clone()).is_ok();
        if !is_db_exist {
            std::fs::create_dir(database_dir).expect("failed to create database directory");
        }

        let conn_pool = block_on(initialize_connection(
            database_file.to_str().unwrap().to_string(),
        ))
        .unwrap();

        if std::fs::metadata(database_file).is_ok() {
            block_on(migrate_db(&conn_pool))
                .expect("failed to migrate db and initialization was failed.");
        }

        SqliteConnectionContext { pool: conn_pool }
    }

    fn get_connection(&self) -> Result<DbConnection, Error> {
        let connection = DbConnection::new(self.pool.clone());
        Ok(connection)
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
