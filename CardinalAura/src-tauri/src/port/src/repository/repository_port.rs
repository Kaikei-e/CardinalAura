use anyhow::Error;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[async_trait::async_trait]
pub trait RepositoryPort {
    fn new() -> Self;
    fn get_connection(&self) -> Result<DbConnection, Error>;
}

pub struct DbConnection {
    pub pool: Arc<Pool<Sqlite>>,
}

impl DbConnection {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        DbConnection {
            pool: Arc::new(pool),
        }
    }
}

pub trait ConnectionContext {
    fn get_connection(&self) -> Result<DbConnection, Error>;
}