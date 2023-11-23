use domain::rss_feed_site::RssFeedSite;
use driver::http_req_driver::HttpClientDriver;
use driver::register_driver::SqliteDriver;
use driver::sqlite_driver::SqliteConnectionContext;
use port::http_client::http_client_port::HttpClientPort;
use port::register::register_feed_url_port::RegisterFeedUrlPort;
use usecase::register_function::register_url_usecase::RegisterSingleUrlUseCase;

#[derive(serde::Serialize)]
struct RssFeedSiteDto {
    url: String,
    title: String,
    description: String,
    link: String,
    items: String,
    item_description: String,
    language: String,
}

impl RssFeedSiteDto {
    fn from(rss_feed_site: RssFeedSite) -> RssFeedSiteDto {
        RssFeedSiteDto {
            url: rss_feed_site.link,
            title: rss_feed_site.title,
            description: rss_feed_site.description,
            link: rss_feed_site.url,
            items: rss_feed_site.items,
            item_description: rss_feed_site.item_description,
            language: rss_feed_site.language,
        }
    }
}

#[tauri::command]
pub fn invoke_register_single_feed_link_command(url: String) -> Result<String, String> {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

        rt.block_on(async {
            let http_client_port: HttpClientDriver = HttpClientPort::new();
            let register_url_port = SqliteDriver::<SqliteConnectionContext>::new();

            let register_single_url_usecase =
                RegisterSingleUrlUseCase::new(http_client_port, register_url_port);

            register_single_url_usecase
                .execute(url)
                .await
                .map_err(|e| e.to_string())
        })
    })
    .join()
    .unwrap_or_else(|_| Err("Failed to execute command".to_string()))
}
