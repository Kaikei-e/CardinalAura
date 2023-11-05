use domain::rss_feed_site::RssFeedSite;
use sqlx::types::uuid;
use sqlx::SqlitePool;

#[derive(sqlx::FromRow)]
struct RssFeedSiteDto {
    id: i64,
    uuid: String,
    xml_version: i64,
    rss_version: i64,
    url: String,
    title: String,
    description: String,
    link: String,
    links: String,
    item_description: String,
    language: String,
    created_at: String,
    updated_at: String,
    feed_category: i64,
    is_active: i64,
    is_favorite: i64,
    is_read: i64,
}

pub async fn register_rss_feed_site(
    pool: SqlitePool,
    rss_feed_url: RssFeedSite,
) -> Result<(), sqlx::Error> {
    let uid = uuid::Uuid::new_v4();
    let row_affected = sqlx::query(
        "INSERT INTO follow_lists
         (uuid, xml_version, rss_version,
          url, title, description,
           link, links, item_description,
            language)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(rss_feed_url.xml_version)
    .bind(rss_feed_url.rss_version)
    .bind(rss_feed_url.url)
    .bind(rss_feed_url.title)
    .bind(rss_feed_url.description)
    .bind(rss_feed_url.link)
    .bind(rss_feed_url.links)
    .bind(rss_feed_url.item_description)
    .bind(rss_feed_url.language)
    .execute(&pool)
    .await?;

    if row_affected.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}
