use crate::db::DBPool;
use crate::models::announcement::Announcement;
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AnnouncementRepository {
    async fn find_by_id(&self, pool: &DBPool, id: &str) -> Result<Announcement>;
}

pub struct AnnouncementRepositoryImpl {}

#[async_trait]
impl AnnouncementRepository for AnnouncementRepositoryImpl {
    async fn find_by_id(&self, pool: &DBPool, id: &str) -> Result<Announcement> {
        let announcement: Announcement =
            sqlx::query_as("SELECT * FROM `announcements` WHERE `id` = ?")
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(announcement)
    }
}
