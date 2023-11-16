use anyhow::Error;
use sqlx::{Pool, Sqlite};

#[async_trait::async_trait]
pub trait RepositoryPort {
    type ConnectionContext;
    fn new() -> Self;
    fn get_connection(&self) -> Result<DbConnection, Error>;
}

pub struct DbConnection {
    pub pool: Pool<Sqlite>,
}

impl DbConnection {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

pub trait ConnectionContext {
    fn get_connection(&self) -> Result<DbConnection, Error>;
}

pub struct SqliteConnectionContext {
    pool: Pool<Sqlite>,
}

impl ConnectionContext for SqliteConnectionContext {
    fn get_connection(&self) -> Result<DbConnection, Error> {
        let connection = DbConnection::new(self.pool.clone());
        Ok(connection)
    }
}
