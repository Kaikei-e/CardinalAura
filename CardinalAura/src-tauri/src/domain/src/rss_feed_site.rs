use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct RssFeedSite {
    pub url: String,
    pub title: String,
    pub description: String,
    pub link: String,
    pub items: String,
    pub item_description: String,
    pub language: String,
}

impl Default for RssFeedSite {
    fn default() -> Self {
        RssFeedSite {
            url: "".to_string(),
            title: "Failed to fetch.".to_string(),
            description: "".to_string(),
            link: "".to_string(),
            items: "".to_string(),
            item_description: "".to_string(),
            language: "".to_string(),
        }
    }
}
