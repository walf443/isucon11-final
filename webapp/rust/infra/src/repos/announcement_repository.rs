use async_trait::async_trait;
use isucholar_core::db::DBConn;
use isucholar_core::models::announcement::{Announcement, AnnouncementID};
use isucholar_core::models::course::CourseID;
use isucholar_core::repos::announcement_repository::AnnouncementRepository;
use isucholar_core::repos::error::ReposError::AnnouncementDuplicate;
use isucholar_core::repos::error::Result;
use isucholar_core::MYSQL_ERR_NUM_DUPLICATE_ENTRY;

#[cfg(test)]
mod create;
#[cfg(test)]
mod find_by_id;

#[derive(Clone)]
pub struct AnnouncementRepositoryInfra {}

#[async_trait]
impl AnnouncementRepository for AnnouncementRepositoryInfra {
    async fn create(&self, conn: &mut DBConn, req: &Announcement) -> Result<()> {
        let result = sqlx::query!(
            "INSERT INTO `announcements` (`id`, `course_id`, `title`, `message`) VALUES (?, ?, ?, ?)",
            &req.id,
            &req.course_id,
            &req.title,
            &req.message,
        )
            .execute(conn)
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

    async fn find_by_id(&self, conn: &mut DBConn, id: &AnnouncementID) -> Result<Announcement> {
        let announcement = sqlx::query_as!(
            Announcement,
            r"
                SELECT
                    id as `id:AnnouncementID`,
                    course_id as `course_id:CourseID`,
                    title,
                    message
                FROM `announcements` WHERE `id` = ?
            ",
            id
        )
        .fetch_one(conn)
        .await?;

        Ok(announcement)
    }
}
