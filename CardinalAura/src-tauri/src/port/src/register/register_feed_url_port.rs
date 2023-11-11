use anyhow::Error;
use domain::rss_feed_site::RssFeedSite;
use mockall::automock;

#[automock]
#[async_trait::async_trait]
pub trait RegisterFeedUrlPort {
    fn new() -> Self;
    async fn register_url(&self, feed: RssFeedSite) -> Result<String, Error>;
}
