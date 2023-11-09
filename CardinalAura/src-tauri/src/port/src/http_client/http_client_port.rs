use anyhow::Error;
use mockall::automock;
use domain::rss_feed_site::RssFeedSite;

#[async_trait::async_trait]
#[automock]
pub trait HttpClientPort {
    fn new() -> Self;
    async fn get(&self, url: String) -> Result<RssFeedSite, Error>;
}

