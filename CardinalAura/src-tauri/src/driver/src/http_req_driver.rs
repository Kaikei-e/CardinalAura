use anyhow::Error;
use domain::rss_feed_site::RssFeedSite;
use port::http_client::http_client_port::HttpClientPort;
use rss::extension::{dublincore, ExtensionMap};
use rss::{Category, Channel, Enclosure, Guid, Source};

#[derive(Debug)]
struct FeedItem {
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    #[cfg_attr(feature = "builders", builder(setter(each = "category")))]
    pub categories: Vec<Category>,
    pub comments: Option<String>,
    pub enclosure: Option<Enclosure>,
    pub guid: Option<Guid>,
    pub pub_date: Option<String>,
    pub source: Option<Source>,
    pub content: Option<String>,
    #[cfg_attr(feature = "builders", builder(setter(each = "extension")))]
    pub extensions: ExtensionMap,
    pub dublin_core_ext: Option<dublincore::DublinCoreExtension>,
}

pub struct HttpClientDriver;

#[async_trait::async_trait]
impl HttpClientPort for HttpClientDriver {
    fn new() -> Self {
        HttpClientDriver
    }

    async fn get(&self, url: String) -> Result<RssFeedSite, Error> {
        let response = reqwest::get(url.clone()).await?.bytes().await?;

        let channel = Channel::read_from(&response[..])?;
        let rss_feed_item = channel
            .clone()
            .items
            .iter()
            .map(|item| FeedItem {
                title: item.title.clone(),
                link: item.link.clone(),
                description: item.description.clone(),

                author: item.author.clone(),
                categories: item.categories.clone(),
                comments: item.comments.clone(),
                enclosure: item.enclosure.clone(),
                guid: item.guid.clone(),
                pub_date: item.pub_date.clone(),
                source: item.source.clone(),
                content: item.content.clone(),
                extensions: item.extensions.clone(),
                dublin_core_ext: item.dublin_core_ext.clone(),
            })
            .map(|item| format!("{:?}", item))
            .collect::<Vec<String>>()
            .join("\n");


        Ok(RssFeedSite {
            url,
            title: channel.clone().title,
            description: channel.clone().description,
            link: channel.clone().link,
            items: rss_feed_item,
            item_description: channel.clone().description,
            language: match channel.clone().language {
                Some(language) => language,
                None => "".to_string(),
            },
        })
    }
}
