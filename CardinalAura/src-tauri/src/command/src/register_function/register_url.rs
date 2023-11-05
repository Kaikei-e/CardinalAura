#[tauri::command]
pub fn register_url_command(url: String) -> String {
    url
}