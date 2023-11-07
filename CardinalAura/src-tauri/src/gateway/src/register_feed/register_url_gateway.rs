use anyhow::Error;
use port::register::register_feed_url_port::RegisterUrlPort;

pub struct SingleFeedUrl(pub String);
pub struct RegisterUrlGateway;

impl RegisterUrlPort for RegisterUrlGateway {
    fn register_single_feed_url(&self, url: String) -> Result<String, Error> {
        todo!("not yet implemented")
    }
}
