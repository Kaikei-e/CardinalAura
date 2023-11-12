use anyhow::Error;
use sqlx::{Pool, Sqlite};

pub trait  RepositoryPort {
    fn new() -> Self;
}

pub struct DbConnection;

pub trait ConnectionContext {
  fn get_connection(&self) -> Result<DbConnection, Error>;
}

pub struct SqliteConnectionContext{
  pool: Pool<Sqlite>
}

impl ConnectionContext for SqliteConnectionContext {
    fn get_connection(&self) -> Result<DbConnection, Error> {
        let connection = self.pool.acquire();

        todo!()
    }
}