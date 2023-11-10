use domain::rss_feed_site::RssFeedSite;
use port::http_client::http_client_port::HttpClientPort;

pub struct RegisterSingleUrlUseCase<T> {
    http_client_port: T,
}

impl<T: HttpClientPort> RegisterSingleUrlUseCase<T> {
    pub fn new(http_client_port: T) -> RegisterSingleUrlUseCase<T> {
        RegisterSingleUrlUseCase { http_client_port }
    }

    pub async fn execute(&self, url: String) -> RssFeedSite {
        let rss_feed_site = self.http_client_port.get(url.clone()).await;

        match rss_feed_site {
            Ok(rss_feed_site) => rss_feed_site,
            Err(_) => todo!("error handling"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use port::http_client::http_client_port::MockHttpClientPort;

    #[tokio::test]
    async fn test_register_single_url() {
        let url = "http://lorem-rss.herokuapp.com/feed".to_string();
        let passing_url = url.clone();

        let mut mock_http_client_port = MockHttpClientPort::default();

        mock_http_client_port
            .expect_get()
            .with(eq(url.clone()))
            .times(1)
            .return_once(move |_| {
                Ok(RssFeedSite {
                    url: url.clone(),
                    title: "lorem".to_string(),
                    description: "hogehoge".to_string(),
                    link: "http://example.com/".to_string(),
                    items: vec![
                        "http://example.com/".to_string(),
                        "http://example.com/".to_string(),
                    ]
                    .iter()
                    .map(|item| item.to_string())
                    .collect(),
                    item_description: "".to_string(),
                    language: "".to_string(),
                })
            });

        let usecase = RegisterSingleUrlUseCase::new(mock_http_client_port);
        let result = usecase.execute(passing_url).await;

        let expected_items = vec![
            "http://example.com/".to_string(),
            "http://example.com/".to_string(),
        ]
        .iter()
        .map(|item| item.to_string())
        .collect();
        let expected = RssFeedSite {
            url: "http://lorem-rss.herokuapp.com/feed".to_string(),
            title: "lorem".to_string(),
            description: "hogehoge".to_string(),
            link: "http://example.com/".to_string(),
            items: expected_items,
            item_description: "".to_string(),
            language: "".to_string(),
        };

        assert_eq!(result, expected);
    }
}
