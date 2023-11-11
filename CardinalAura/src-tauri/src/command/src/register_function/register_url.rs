use usecase::register_function::register_url_usecase::RegisterSingleUrlUseCase;

#[tauri::command]
pub fn register_single_url_command(url: String) -> String {
    let uc = RegisterSingleUrlUseCase::new();
    let result = uc.execute(url);

    match result {
        Ok(_) => "success".to_string(),
        Err(_) => "error".to_string(),
    }
}