use driver::http_req_driver::HttpClientDriver;
use driver::register_driver::SqliteDriver;
use sqlx::{Pool, Sqlite};
use usecase::register_function::register_url_usecase::RegisterSingleUrlUseCase;

pub struct DIUsecase {
    register_single_url_usecase: RegisterSingleUrlUseCase<HttpClientDriver, SqliteDriver>,
}

pub fn dependency_injection(pool: Pool<Sqlite>) -> DIUsecase {
    let db_driver = SqliteDriver::new(pool);
    let http_driver = HttpClientDriver::new();

    let register_single_url_usecase = RegisterSingleUrlUseCase::new(http_driver, db_driver);

    let usecase = DIUsecase {
        register_single_url_usecase,
    };
    (usecase)
}
