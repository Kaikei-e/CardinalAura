use anyhow::Error;
use rss::Channel;

#[async_trait::async_trait]
pub trait HttpClientPort {
    async fn get(&self, url: String) -> Result<Channel, Error>;
}