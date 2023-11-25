use std::sync::Arc;

use chrono::Utc;
use domain::rss_feed_site::RssFeedSite;
use port::register::register_feed_url_port::RegisterFeedUrlPort;
use port::repository::repository_port::RepositoryPort;
use sqlx::types::uuid;
use sqlx::SqlitePool;

#[derive(sqlx::FromRow, Default)]
pub struct RssFeedSiteDtoWrite {
    pub uuid: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub link: String,
    pub links: String,
    pub item_description: String,
    pub language: String,
    pub updated_at: chrono::DateTime<Utc>,
    pub created_at: chrono::DateTime<Utc>,
}

pub struct SqliteDriver<R: RepositoryPort> {
    repository: R,
}

#[async_trait::async_trait]
impl<R: RepositoryPort + std::marker::Sync> RegisterFeedUrlPort for SqliteDriver<R> {
    fn new() -> SqliteDriver<R> {
        SqliteDriver {
            repository: R::new(),
        }
    }

    async fn register_url(&self, feed: RssFeedSite) -> Result<String, anyhow::Error> {
        let connection = self.repository.get_connection();

        if let Err(e) = connection {
            panic!("Failed to get connection from pool: {}", e);
        }

        let dto = RssFeedSiteDtoWrite::from(feed);
        let url = dto.url.clone();

        if let Err(e) = connection {
            panic!("Failed to get connection from pool: {}", e);
        }

        let result = register_rss_feed_site(connection.unwrap().pool, dto).await;
        match result {
            Ok(_) => Ok(url.to_string()),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }
}

impl RssFeedSiteDtoWrite {
    fn default() -> Self {
        RssFeedSiteDtoWrite {
            uuid: "".to_string(),
            url: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            link: "".to_string(),
            links: "".to_string(),
            item_description: "".to_string(),
            language: "".to_string(),
            updated_at: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
        }
    }

    fn from(feed: RssFeedSite) -> Self {
        let now = chrono::Utc::now();
        let now_updated_at = now.clone();

        RssFeedSiteDtoWrite {
            uuid: uuid::Uuid::new_v4().to_string(),
            url: feed.url,
            title: feed.title,
            description: feed.description,
            link: feed.link,
            links: feed.items,
            item_description: feed.item_description,
            language: feed.language,
            updated_at: now_updated_at,
            created_at: now,
        }
    }
}

pub async fn register_rss_feed_site(
    pool: Arc<SqlitePool>,
    rss_feed: RssFeedSiteDtoWrite,
) -> Result<(), sqlx::Error> {
    let uid = uuid::Uuid::new_v4();
    let now = chrono::Utc::now();
    let now_updated_at = now.clone();

    let row_affected = sqlx::query(
        "INSERT INTO follow_lists
        (uuid, url, title, description,
        link, links, item_description, language,
        updated_at, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(rss_feed.url)
    .bind(rss_feed.title)
    .bind(rss_feed.description)
    .bind(rss_feed.link)
    .bind(rss_feed.links)
    .bind(rss_feed.item_description)
    .bind(rss_feed.language)
    .bind(now_updated_at.to_rfc3339())
    .bind(now.to_rfc3339())
    .execute(&*pool)
    .await?;

    if row_affected.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}
