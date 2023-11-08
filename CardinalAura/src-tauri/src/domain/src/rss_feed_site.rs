use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct RssFeedSite {
    pub url: String,
    pub title: String,
    pub description: String,
    pub link: String,
    pub items: String,
    pub item_description: String,
    pub language: String,
}
