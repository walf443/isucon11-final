use async_trait::async_trait;
use isucholar_core::db::DBConn;
use isucholar_core::models::announcement::{AnnouncementID, AnnouncementWithoutDetail};
use isucholar_core::models::announcement_detail::AnnouncementDetail;
use isucholar_core::models::course::CourseID;
use isucholar_core::models::user::UserID;
use isucholar_core::repos::error::Result;
use isucholar_core::repos::unread_announcement_repository::UnreadAnnouncementRepository;
use sqlx::Arguments;

#[cfg(test)]
mod count_unread_by_user_id;
#[cfg(test)]
mod create;
#[cfg(test)]
mod find_announcement_detail_by_announcement_id_and_user_id;
#[cfg(test)]
mod find_unread_announcements_by_user_id;
#[cfg(test)]
mod mark_read;

#[derive(Clone)]
pub struct UnreadAnnouncementRepositoryInfra {}

#[async_trait]
impl UnreadAnnouncementRepository for UnreadAnnouncementRepositoryInfra {
    async fn create(
        &self,
        conn: &mut DBConn,
        announcement_id: &AnnouncementID,
        user_id: &UserID,
    ) -> Result<()> {
        sqlx::query!(
            "INSERT INTO `unread_announcements` (`announcement_id`, `user_id`) VALUES (?, ?)",
            announcement_id,
            user_id
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    async fn mark_read(
        &self,
        conn: &mut DBConn,
        announcement_id: &AnnouncementID,
        user_id: &UserID,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE `unread_announcements` SET `is_deleted` = true WHERE `announcement_id` = ? AND `user_id` = ?",
            announcement_id,
            user_id,
        )
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn count_unread_by_user_id(&self, conn: &mut DBConn, user_id: &UserID) -> Result<i64> {
        let unread_count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM `unread_announcements` WHERE `user_id` = ? AND NOT `is_deleted`",
            user_id
        )
        .fetch_one(conn)
        .await?;

        Ok(unread_count)
    }

    async fn find_unread_announcements_by_user_id(
        &self,
        tx: &mut DBConn,
        user_id: &UserID,
        limit: i64,
        offset: i64,
        course_id: Option<CourseID>,
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
        args.add(user_id);
        args.add(user_id);
        args.add(limit + 1);
        args.add(offset);

        let announcements: Vec<AnnouncementWithoutDetail> =
            sqlx::query_as_with(&query, args).fetch_all(tx).await?;

        Ok(announcements)
    }

    async fn find_announcement_detail_by_announcement_id_and_user_id(
        &self,
        conn: &mut DBConn,
        announcement_id: &AnnouncementID,
        user_id: &UserID,
    ) -> Result<Option<AnnouncementDetail>> {
        let announcement: Option<AnnouncementDetail> = sqlx::query_as!(
            AnnouncementDetail,
            r"
                SELECT
                    `announcements`.`id` as `id:AnnouncementID`,
                    `courses`.`id` AS `course_id:CourseID`,
                    `courses`.`name` AS `course_name`,
                    `announcements`.`title`,
                    `announcements`.`message`,
                    NOT `unread_announcements`.`is_deleted` AS `unread:bool`
                FROM `announcements`
                JOIN `courses` ON `courses`.`id` = `announcements`.`course_id`
                JOIN `unread_announcements` ON `unread_announcements`.`announcement_id` = `announcements`.`id`
                WHERE
                    `announcements`.`id` = ? AND `unread_announcements`.`user_id` = ?
            ",
            announcement_id,
            user_id,
        ).fetch_optional(conn).await?;

        Ok(announcement)
    }
}
