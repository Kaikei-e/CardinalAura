use anyhow::Error;
use domain::rss_feed_site::RssFeedSite;
use mockall::automock;

#[automock]
pub trait HttpClientPort {
    fn new() -> Self;
    fn get(&self, url: String) -> Result<RssFeedSite, Error>;
}
