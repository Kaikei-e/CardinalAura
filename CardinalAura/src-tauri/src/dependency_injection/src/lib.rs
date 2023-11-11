use driver::sqlite_driver::SqliteDriver;
use sqlx::{Pool, Sqlite};

pub fn dependency_injection(pool: Pool<Sqlite>) -> SqliteDriver {

    let driver = SqliteDriver::new(pool);

    // by intentionally returning variable
    driver
}
