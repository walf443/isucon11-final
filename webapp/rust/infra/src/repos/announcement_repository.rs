use async_trait::async_trait;
use isucholar_core::db::{DBPool, TxConn};
use isucholar_core::models::announcement::Announcement;
use isucholar_core::repos::announcement_repository::AnnouncementRepository;
use isucholar_core::repos::error::ReposError::AnnouncementDuplicate;
use isucholar_core::repos::error::Result;
use isucholar_core::MYSQL_ERR_NUM_DUPLICATE_ENTRY;

#[derive(Clone)]
pub struct AnnouncementRepositoryImpl {}

#[async_trait]
impl AnnouncementRepository for AnnouncementRepositoryImpl {
    async fn create_in_tx<'c>(&self, tx: &mut TxConn<'c>, req: &Announcement) -> Result<()> {
        let result = sqlx::query(
            "INSERT INTO `announcements` (`id`, `course_id`, `title`, `message`) VALUES (?, ?, ?, ?)",
        )
            .bind(&req.id)
            .bind(&req.course_id)
            .bind(&req.title)
            .bind(&req.message)
            .execute(tx)
            .await;

        if let Err(e) = result {
            if let sqlx::error::Error::Database(ref db_error) = e {
                if let Some(mysql_error) =
                    db_error.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>()
                {
                    if mysql_error.number() == MYSQL_ERR_NUM_DUPLICATE_ENTRY {
                        return Err(AnnouncementDuplicate);
                    }
                }
            }
            return Err(e.into());
        }

        Ok(())
    }

    async fn find_by_id(&self, pool: &DBPool, id: &str) -> Result<Announcement> {
        let announcement: Announcement =
            sqlx::query_as("SELECT * FROM `announcements` WHERE `id` = ?")
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(announcement)
    }
}
