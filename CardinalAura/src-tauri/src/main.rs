// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use driver::sqlite_driver::initialize_connection;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use command::register_function::register_url;

fn main() {
    use tauri::async_runtime::block_on;

    let maybe_conn_pool = block_on(initialize_connection("core_db.sqlite".to_string()));

    if let Err(e) = maybe_conn_pool {
        panic!("Failed to get connection from pool: {}", e);
    }

    let conn_pool = maybe_conn_pool.unwrap();

    tauri::Builder::default()
        .manage(conn_pool) // Store the connection pool in Tauri's state
        .invoke_handler(tauri::generate_handler![
            greet,
            register_url::invoke_register_single_feed_link_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
