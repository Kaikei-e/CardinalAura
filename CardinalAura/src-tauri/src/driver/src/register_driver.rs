use domain::rss_feed_site::RssFeedSite;
use sqlx::types::uuid;
use sqlx::SqlitePool;

#[derive(sqlx::FromRow, Default)]
struct RssFeedSiteDto {
    pub uuid: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub link: String,
    pub links: String,
    pub item_description: String,
    pub language: String,
    pub created_at: String,
    pub updated_at: String,
    pub feed_category: i64,
    pub is_active: i64,
    pub is_favorite: i64,
    pub is_read: i64,
}

impl RssFeedSiteDto {
    fn default() -> Self {
        let now = chrono::Local::now().to_string();

        RssFeedSiteDto {
            uuid: "".to_string(),
            url: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            link: "".to_string(),
            links: "".to_string(),
            item_description: "".to_string(),
            language: "".to_string(),
            created_at: now.clone(),
            updated_at: now,
            feed_category: 0,
            is_active: 0,
            is_favorite: 0,
            is_read: 0,
        }
    }
}

pub async fn register_rss_feed_site(
    pool: SqlitePool,
    rss_feed_url: RssFeedSite,
) -> Result<(), sqlx::Error> {
    let uid = uuid::Uuid::new_v4();
    let row_affected = sqlx::query(
        "INSERT INTO follow_lists
         (uuid, url, title, description,
           link, links, item_description, language)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(rss_feed_url.url)
    .bind(rss_feed_url.title)
    .bind(rss_feed_url.description)
    .bind(rss_feed_url.link)
    .bind(rss_feed_url.items)
    .bind(rss_feed_url.item_description)
    .bind(rss_feed_url.language)
    .execute(&pool)
    .await?;

    if row_affected.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}
