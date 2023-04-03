use crate::db;
use crate::db::TxConn;
use crate::models::announcement::AnnouncementWithoutDetail;
use crate::repos::error::Result;
use async_trait::async_trait;
use sqlx::Arguments;

#[async_trait]
pub trait UnreadAnnouncementRepository {
    async fn create_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        announcement_id: &str,
        user_id: &str,
    ) -> Result<()>;
    async fn mark_read<'c>(&self, tx: &mut TxConn<'c>, id: &str, user_id: &str) -> Result<()>;
    async fn count_unread_by_user_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
    ) -> Result<i64>;
    async fn find_unread_announcements_by_user_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        limit: i64,
        offset: i64,
        course_id: Option<&str>,
    ) -> Result<Vec<AnnouncementWithoutDetail>>;
}

pub struct UnreadAnnouncementRepositoryImpl {}

#[async_trait]
impl UnreadAnnouncementRepository for UnreadAnnouncementRepositoryImpl {
    async fn create_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        announcement_id: &str,
        user_id: &str,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO `unread_announcements` (`announcement_id`, `user_id`) VALUES (?, ?)",
        )
        .bind(announcement_id)
        .bind(user_id)
        .execute(tx)
        .await?;
        Ok(())
    }
    async fn mark_read<'c>(
        &self,
        tx: &mut TxConn<'c>,
        announcement_id: &str,
        user_id: &str,
    ) -> Result<()> {
        sqlx::query("UPDATE `unread_announcements` SET `is_deleted` = true WHERE `announcement_id` = ? AND `user_id` = ?")
            .bind(announcement_id)
            .bind(user_id)
            .execute(tx)
            .await?;

        Ok(())
    }

    async fn count_unread_by_user_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
    ) -> Result<i64> {
        let unread_count: i64 = db::fetch_one_scalar(
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM `unread_announcements` WHERE `user_id` = ? AND NOT `is_deleted`",
            )
                .bind(user_id),
            tx,
        )
            .await?;

        Ok(unread_count)
    }

    async fn find_unread_announcements_by_user_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        limit: i64,
        offset: i64,
        course_id: Option<&str>,
    ) -> Result<Vec<AnnouncementWithoutDetail>> {
        let mut query = concat!(
        "SELECT `announcements`.`id`, `courses`.`id` AS `course_id`, `courses`.`name` AS `course_name`, `announcements`.`title`, NOT `unread_announcements`.`is_deleted` AS `unread`",
        " FROM `announcements`",
        " JOIN `courses` ON `announcements`.`course_id` = `courses`.`id`",
        " JOIN `registrations` ON `courses`.`id` = `registrations`.`course_id`",
        " JOIN `unread_announcements` ON `announcements`.`id` = `unread_announcements`.`announcement_id`",
        " WHERE 1=1",
        ).to_owned();
        let mut args = sqlx::mysql::MySqlArguments::default();
        if let Some(course_id) = course_id {
            query.push_str(" AND `announcements`.`course_id` = ?");
            args.add(course_id);
        }
        query.push_str(concat!(
            " AND `unread_announcements`.`user_id` = ?",
            " AND `registrations`.`user_id` = ?",
            " ORDER BY `announcements`.`id` DESC",
            " LIMIT ? OFFSET ?",
        ));
        args.add(&user_id);
        args.add(&user_id);
        args.add(limit + 1);
        args.add(offset);

        let announcements: Vec<AnnouncementWithoutDetail> =
            sqlx::query_as_with(&query, args).fetch_all(tx).await?;

        Ok(announcements)
    }
}
