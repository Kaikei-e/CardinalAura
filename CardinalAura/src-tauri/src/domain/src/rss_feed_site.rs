use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct RssFeedSite {
    pub xml_version: i64,
    pub rss_version: i64,
    pub url: String,
    pub title: String,
    pub description: String,
    pub link: String,
    pub links: String,
    pub item_description: String,
    pub language: String,
}
