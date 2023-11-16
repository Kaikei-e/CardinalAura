use port::http_client::http_client_port::HttpClientPort;
use port::repository::repository_port::{ConnectionContext, DbConnection, RepositoryPort};
use sqlx::{Pool, Sqlite};
use usecase::register_function::register_url_usecase::RegisterSingleUrlUseCase;

// Assuming these are the concrete implementations for the traits.
use driver::http_req_driver::HttpClientDriver;
use driver::register_driver::SqliteDriver;

pub struct DIUsecase {
    pub register_single_url_usecase:
        RegisterSingleUrlUseCase<HttpClientDriver, 
            SqliteDriver::<Sqlite>,
        >,
}
pub fn dependency_injection(pool: Pool<Sqlite>) -> DIUsecase {
    // Instantiate the concrete implementations of the traits.
    let http_driver = HttpClientDriver::new(); // Assumes `new` does not require parameters.
    let db_driver = SqliteDriver::new(pool); // Passes the SQL pool to the DB driver.

    // Create the use case with its dependencies injected.
    let register_single_url_usecase = RegisterSingleUrlUseCase::new(http_driver, db_driver);

    // Construct the DI container with the use case.
    let usecase_container = DIUsecase {
        register_single_url_usecase,
    };

    usecase_container
}
