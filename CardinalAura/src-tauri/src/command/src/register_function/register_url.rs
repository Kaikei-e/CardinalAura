use domain::rss_feed_site::RssFeedSite;
use driver::http_req_driver::HttpClientDriver;
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
            url: rss_feed_site.url,
            title: rss_feed_site.title,
            description: rss_feed_site.description,
            link: rss_feed_site.link,
            items: rss_feed_site.items,
            item_description: rss_feed_site.item_description,
            language: rss_feed_site.language,
        }
    }
}

#[tauri::command]
pub fn invoke(url: String) -> Result<String, anyhow::Error> {
    let http_client_port = HttpClientDriver {};
    let register_url_usecase = RegisterSingleUrlUseCase::new(http_client_port);

    let result = tauri::async_runtime::block_on(register_url_usecase.execute(url));

    match result {
        Ok(result) => {
            let rss_feed_site_dto = RssFeedSiteDto::from(result);
            let json = serde_json::to_string(&rss_feed_site_dto)?;
            Ok(json)
        }
        Err(_) => Err(anyhow::anyhow!("Failed to register URL")),
    }
}
