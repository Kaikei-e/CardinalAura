// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use command::register_function::register_url;
use driver::sqlite_driver::SqliteConnectionContext;
use port::repository::repository_port::RepositoryPort;

fn main() {
    // dependency_injection::dependency_injection(conn_pool.clone());

    // For lazy static initialization
    let repository_port = SqliteConnectionContext::new();
    let conn_pool = repository_port.get_connection().unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            register_url::invoke_register_single_feed_link_command
        ])
        .setup(|app| {
            app.manage(conn_pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
