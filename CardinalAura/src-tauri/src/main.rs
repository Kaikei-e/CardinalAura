// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
mod driver;

fn main() {
    use tauri::async_runtime::block_on;

    let dir_path = dotenv::var("DIRECTORY_PATH").expect("DIRECTORY_PATH must be set"));
    if dir_path.is_none() {
        println!("Error loading .env file");
    }

    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let is_db_exist = std::fs::metadata("core_db.sqlite").is_ok();
    if !is_db_exist {
        let result = std::fs::create_dir("core_db.sqlite");
        if result.is_err() {
            println!("Error creating core_db.sqlite directory: {:?}", result);
        }
    }

    let conn_pool = block_on(driver::core_db::sqlite_driver::initialize_connection(&db_url));
    if conn_pool.is_err() {
        println!("Error initializing connection pool: {:?}", conn_pool);
    }

    if !is_db_exist{
        block_on(driver::core_db::sqlite_driver::migrate_db(
            &conn_pool.unwrap()
        )
        )
            .expect("failed to migrate db and initialization was failed.");
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            app.manage(conn_pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
