// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


use driver::sqlite_driver;

fn main() {
    use tauri::async_runtime::block_on;

    const DATABASE_DIR: &str = "driver_old/db/data";
    const DATABASE_FILE: &str = "core_db.sqlite";

    let base_dir = std::env::current_dir().expect("Can't access the current directory");

    let app_dir = base_dir.join("src");

    let database_dir = app_dir.join(DATABASE_DIR);
    let database_file = database_dir.join(DATABASE_FILE);

    println!("Database file: {:?}", database_file);

    let is_db_exist = std::fs::metadata(database_dir.clone()).is_ok();
    if !is_db_exist {
        std::fs::create_dir(database_dir).expect("failed to create database directory");
    }

    // let database_url = format!("sqlite://{}", database_file.to_str().unwrap());

    let conn_pool = block_on(sqlite_driver::initialize_connection(
        database_file.to_str().unwrap().to_string(),
    ))
    .unwrap();

    if std::fs::metadata(database_file).is_ok() {
        block_on(sqlite_driver::migrate_db(&conn_pool))
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
