// use anyhow::Error;
// use domain::rss_feed_site::RssFeedSite;
// use driver::register_driver::register_rss_feed_site;
// use port::register::register_feed_url_port::RegisterFeedUrlPort;

// pub struct RegisterUrlGateway;

// #[async_trait::async_trait]
// impl RegisterFeedUrlPort for RegisterUrlGateway {
//     fn new() -> Self {
//         RegisterUrlGateway
//     }

//     async fn register_url(&self, feed: RssFeedSite) -> Result<String, Error> {
//         let result = register_rss_feed_site(feed).await;

//         todo!()
//     }
// }

// // In the port layer
// pub trait FeedUrlPort {
//     async fn register_url(&self, feed: RssFeedSite) -> Result<String, Error>;
// }

// // In the driver layer
// pub struct SqliteDriver;

// #[async_trait::async_trait]
// impl FeedUrlPort for SqliteDriver {
//     async fn register_url(&self, feed: RssFeedSite) -> Result<String, Error> {
//         // Implementation goes here
//     }
// }

// // In the use case layer
// pub struct RegisterSingleUrlUseCase<T: FeedUrlPort> {
//     feed_url_port: T,
// }

// impl<T: FeedUrlPort> RegisterSingleUrlUseCase<T> {
//     pub fn new(feed_url_port: T) -> Self {
//         Self { feed_url_port }
//     }

//     pub async fn register_url(&self, feed: RssFeedSite) -> Result<String, Error> {
//         self.feed_url_port.register_url(feed).await
//     }
// }
