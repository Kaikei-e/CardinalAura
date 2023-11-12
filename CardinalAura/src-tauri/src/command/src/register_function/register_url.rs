use domain::rss_feed_site::RssFeedSite;
use usecase::register_function::register_url_usecase::RegisterSingleUrlUseCase;
use port::http_client::http_client_port::{HttpClientPort};
use port::register::register_feed_url_port::RegisterFeedUrlPort;

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
pub fn invoke_register_single_feed_link_command(url: String) -> String {
    let http_client_port = HttpClientPort::new();
    let register_url_port = RegisterFeedUrlPort::new();
    let register_url_usecase = RegisterSingleUrlUseCase::new(http_client_port, register_url_port);

    let registered_link = tauri::async_runtime::block_on(register_url_usecase.execute(url));

    match registered_link {
        Ok(result) => {
            let registered_link_json = serde_json::from_str(&result);
            match registered_link_json {
                Ok(js) => js,
                Err(_) => todo!(),
            }
        }
        Err(_) => "Failed to register URL!".to_string(),
    }
}
