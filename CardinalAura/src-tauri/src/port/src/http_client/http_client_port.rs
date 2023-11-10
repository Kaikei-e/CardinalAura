use anyhow::Error;
use domain::rss_feed_site::RssFeedSite;
use mockall::automock;

#[automock]
#[async_trait::async_trait]
pub trait HttpClientPort {
    fn new() -> Self;
    async fn get(&self, url: String) -> Result<RssFeedSite, Error>;
}
