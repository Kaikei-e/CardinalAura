use anyhow::Error;
use domain::rss_feed_site::RssFeedSite;
use port::register::register_feed_url_port::RegisterFeedUrlPort;
use driver::register_driver::register_rss_feed_site;
use driver::sqlite_driver::SqliteDriver;

pub struct RegisterUrlGateway;

#[async_trait::async_trait]
impl RegisterFeedUrlPort for RegisterUrlGateway {
    fn new() -> Self {
        RegisterUrlGateway
    }

    async fn register_url(&self, feed: RssFeedSite) -> Result<String, Error> {

        let resutl = register_rss_feed_site(feed).await;

        todo!()
    }
}
