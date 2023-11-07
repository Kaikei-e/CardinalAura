use anyhow::Error;

pub trait RegisterUrlPort {
    fn register_single_feed_url(&self, url: String) -> Result<String, Error>;
}
