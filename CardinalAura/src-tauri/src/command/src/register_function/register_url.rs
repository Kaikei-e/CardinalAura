use usecase::register_function::register_url_usecase::RegisterSingleUrlUseCase;
use port::http_client::http_client_port::HttpClientPort;
pub struct RegisterSingleFeedUrlCommand {
    pub url: String,
}

impl RegisterSingleFeedUrlCommand {
    pub fn new(url: String) -> Self {
        RegisterSingleFeedUrlCommand { url }
    }

    #[tauri::command]
    pub fn invoke(&self, url: String) -> Result<String, anyhow::Error> {
        let http_client_port = HttpClientPort::new();
        let register_url_usecase = RegisterSingleUrlUseCase::new(http_client_port);

        let result = register_url_usecase.register_single_feed_url(url.clone());
        match result {
            Ok(_) => Ok(url.to_string()),
            Err(_) => Err(anyhow::Error::msg("error")),
        }
    }
}