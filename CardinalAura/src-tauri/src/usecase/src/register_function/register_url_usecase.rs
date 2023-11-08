use domain::rss_feed_site::RssFeedSite;

pub fn register_single_url(url: String) -> RssFeedSite {
    todo!("not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_single_url() {
        let url = "http://lorem-rss.herokuapp.com/feed".to_string();


        let result = register_single_url(url.clone());

        let links = vec!["http://example.com/".to_string()];


        let expected = RssFeedSite {
            url: url.to_string(),
            title: "lorem".to_string(),
            description: "hogehoge".to_string(),
            link: "http://example.com/".to_string(),
            items: "http://example.com/".to_string(),
            item_description: "".to_string(),
            language: "".to_string(),
        };

        assert_eq!(result, expected);
    }
}