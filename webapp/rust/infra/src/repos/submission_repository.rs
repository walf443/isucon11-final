use async_trait::async_trait;
use isucholar_core::db::DBConn;
use isucholar_core::models::class::ClassID;
use isucholar_core::models::submission::{CreateSubmission, SubmissionWithUserCode};
use isucholar_core::models::user::{UserCode, UserID};
use isucholar_core::repos::error::Result;
use isucholar_core::repos::submission_repository::SubmissionRepository;

#[cfg(test)]
mod count_by_class_id;
#[cfg(test)]
mod create_or_update;
#[cfg(test)]
mod find_all_with_user_code_by_class_id;
#[cfg(test)]
mod find_score_by_class_id_and_user_id;
#[cfg(test)]
mod update_score_by_user_code_and_class_id;

#[derive(Clone)]
pub struct SubmissionRepositoryInfra {}

#[async_trait]
impl SubmissionRepository for SubmissionRepositoryInfra {
    async fn create_or_update(
        &self,
        conn: &mut DBConn,
        submission: &CreateSubmission,
    ) -> Result<()> {
        sqlx::query!(
            "INSERT INTO `submissions` (`user_id`, `class_id`, `file_name`) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE `file_name` = VALUES(`file_name`)",
            &submission.user_id,
            &submission.class_id,
            &submission.file_name,
        )
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn count_by_class_id(&self, conn: &mut DBConn, class_id: &ClassID) -> Result<i64> {
        let submissions_count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM `submissions` WHERE `class_id` = ?",
            class_id
        )
        .fetch_one(conn)
        .await?;
        Ok(submissions_count)
    }

    async fn update_score_by_user_code_and_class_id(
        &self,
        conn: &mut DBConn,
        user_code: &UserCode,
        class_id: &ClassID,
        score: i64,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE `submissions` JOIN `users` ON `users`.`id` = `submissions`.`user_id` SET `score` = ? WHERE `users`.`code` = ? AND `class_id` = ?",
            score,
            user_code,
            class_id,
        )
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn find_score_by_class_id_and_user_id(
        &self,
        conn: &mut DBConn,
        class_id: &ClassID,
        user_id: &UserID,
    ) -> Result<Option<u8>> {
        let score: Option<Option<u8>> = sqlx::query_scalar!(
            r"
                SELECT `submissions`.`score` FROM `submissions`
                WHERE `user_id` = ? AND `class_id` = ?
            ",
            user_id,
            class_id,
        )
        .fetch_optional(conn)
        .await?;

        match score {
            None => Ok(None),
            Some(Some(score)) => Ok(Some(score)),
            Some(None) => Ok(None),
        }
    }

    async fn find_all_with_user_code_by_class_id(
        &self,
        conn: &mut DBConn,
        class_id: &ClassID,
    ) -> Result<Vec<SubmissionWithUserCode>> {
        let submissions: Vec<SubmissionWithUserCode> = sqlx::query_as!(
            SubmissionWithUserCode,
            r"
                SELECT
                  `submissions`.`user_id` as `user_id:UserID`,
                  `users`.`code` AS `user_code:UserCode`,
                  `submissions`.`file_name`
                FROM `submissions`
                JOIN `users` ON `users`.`id` = `submissions`.`user_id`
                WHERE `class_id` = ?
            ",
            class_id
        )
        .fetch_all(conn)
        .await?;

        Ok(submissions)
    }
}
