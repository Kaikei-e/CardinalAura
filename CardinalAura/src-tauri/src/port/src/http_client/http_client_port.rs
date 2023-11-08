use anyhow::Error;
use domain::rss_feed_site::RssFeedSite;

#[async_trait::async_trait]
pub trait HttpClientPort {
    async fn get(&self, url: String) -> Result<RssFeedSite, Error>;
}