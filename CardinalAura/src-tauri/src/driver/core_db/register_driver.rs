use sqlx::types::uuid;
use sqlx::{Row, SqlitePool};

#[derive(sqlx::FromRow)]
struct RssFeedSite {
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
    rss_feed_url: String,
) -> Result<(), sqlx::Error> {
    // let uid = uuid::Uuid::new_v4();
    // let row_affected: (i64,) = sqlx::query(
    //     "INSERT INTO follow_lists
    //      (uuid, xml_version, rss_version,
    //       url, title, description,
    //        link, links, item_description,
    //         language, created_at, updated_at,
    //          feed_category, is_active, is_favorite,
    //           is_read)
    //      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
    //     )",
    // )
    // .bind(uid)
    // .bind(rss_feed_url)
    //     .execute(&pool)
    // .await?;

    Ok(())
}
