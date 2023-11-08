use domain::rss_feed_site::RssFeedSite;

trait IRegisterSingleUrlUseCase {
    fn execute(&self, url: String) -> RssFeedSite;
}

pub struct RegisterSingleUrlUseCase;

impl IRegisterSingleUrlUseCase for RegisterSingleUrlUseCase {
    fn execute(&self, url: String) -> RssFeedSite {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_single_url() {
        let url = "http://lorem-rss.herokuapp.com/feed".to_string();

        let usecase = RegisterSingleUrlUseCase;

        let result = usecase.execute(url.clone());

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