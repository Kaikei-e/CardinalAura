use domain::rss_feed_site::RssFeedSite;
use port::http_client::http_client_port::HttpClientPort;
use port::register::register_feed_url_port::RegisterFeedUrlPort;

use anyhow::Error;
use std::result::Result;

pub struct RegisterSingleUrlUseCase<T1, T2> {
    http_client_port: T1,
    register_url_port: T2,
}

impl<T1: HttpClientPort, T2: RegisterFeedUrlPort> RegisterSingleUrlUseCase<T1, T2> {
    pub fn new(http_client_port: T1, register_url_port: T2) -> RegisterSingleUrlUseCase<T1, T2> {
        RegisterSingleUrlUseCase {
            http_client_port,
            register_url_port,
        }
    }

    pub async fn execute(&self, url: String) -> Result<String, Error> {
        let rss_feed_site = self.http_client_port.get(url.clone()).await?;
        let registered_rss = self.register_url_port.register_url(rss_feed_site).await;

        match registered_rss {
            Ok(rss_link) => Ok(rss_link),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Ok;
    use mockall::predicate::*;
    use port::http_client::http_client_port::MockHttpClientPort;
    use port::register::register_feed_url_port::MockRegisterFeedUrlPort;

    #[tokio::test]
    async fn test_register_single_url() {
        let url = "https://example.com/feed".to_string();
        let passing_url = url.clone();
        let passing_url2 = url.clone();

        let mut mock_http_client_port = MockHttpClientPort::default();
        let mut mock_register_url_port = MockRegisterFeedUrlPort::default();

        mock_http_client_port
            .expect_get()
            .with(eq(url.clone()))
            .times(1)
            .return_once(move |_| {
                Ok(RssFeedSite {
                    url: url.clone(),
                    title: "example".to_string(),
                    description: "hogehoge".to_string(),
                    link: "https://example.com/".to_string(),
                    items: vec![
                        "https://example.com/".to_string(),
                        "https://example.com/".to_string(),
                    ]
                    .iter()
                    .map(|item| item.to_string())
                    .collect(),
                    item_description: "".to_string(),
                    language: "".to_string(),
                })
            });

        mock_register_url_port
            .expect_register_url()
            .with(eq(RssFeedSite {
                url: passing_url2.clone(),
                title: "example".to_string(),
                description: "hogehoge".to_string(),
                link: "https://example.com/".to_string(),
                items: vec![
                    "https://example.com/".to_string(),
                    "https://example.com/".to_string(),
                ]
                .iter()
                .map(|item| item.to_string())
                .collect(),
                item_description: "".to_string(),
                language: "".to_string(),
            }))
            .times(1)
            .return_once(move |_| Ok("https://example.com".to_string()));

        let usecase = RegisterSingleUrlUseCase::new(mock_http_client_port, mock_register_url_port);
        let result = usecase.execute(passing_url).await.unwrap();
        let expected = "https://example.com";

        assert_eq!(result, expected);
    }
}
