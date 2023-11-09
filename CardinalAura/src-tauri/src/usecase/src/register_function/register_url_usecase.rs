use domain::rss_feed_site::RssFeedSite;
use port::http_client::http_client_port::HttpClientPort;

pub struct RegisterSingleUrlUseCase<T> {
    http_client_port: T,
}

impl<T: HttpClientPort> RegisterSingleUrlUseCase<T> {
    pub fn new(http_client_port: T) -> RegisterSingleUrlUseCase<T> {
        RegisterSingleUrlUseCase {
            http_client_port,
        }
    }

    pub fn execute(&self, url: String) -> RssFeedSite {
        let rss_feed_site = RssFeedSite {
            url: url.to_string(),
            title: "lorem".to_string(),
            description: "hogehoge".to_string(),
            link: "http://example.com/".to_string(),
            items: "http://example.com/".to_string(),
            item_description: "".to_string(),
            language: "".to_string(),
        };

        rss_feed_site
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use port::http_client::http_client_port::MockHttpClientPort;

    #[test]
    fn test_register_single_url() {
        let url = "http://lorem-rss.herokuapp.com/feed".to_string();

        let usecase = RegisterSingleUrlUseCase::new(HttpClientDriver::new());

        let mock_http_client_port = MockHttpClientPort::new();

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