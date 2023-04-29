use async_trait::async_trait;
use isucholar_core::db::{DBConn, TxConn};
use isucholar_core::models::submission::{CreateSubmission, Submission};
use isucholar_core::repos::error::Result;
use isucholar_core::repos::submission_repository::SubmissionRepository;

#[cfg(test)]
mod count_by_class_id;
#[cfg(test)]
mod create_or_update;
#[cfg(test)]
mod find_score_by_class_id_and_user_id;
#[cfg(test)]
mod update_score_by_user_code_and_class_id;

#[derive(Clone)]
pub struct SubmissionRepositoryInfra {}

#[async_trait]
impl SubmissionRepository for SubmissionRepositoryInfra {
    async fn create_or_update<'c>(
        &self,
        tx: &mut TxConn,
        submission: &CreateSubmission,
    ) -> Result<()> {
        sqlx::query!(
            "INSERT INTO `submissions` (`user_id`, `class_id`, `file_name`) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE `file_name` = VALUES(`file_name`)",
            &submission.user_id,
            &submission.class_id,
            &submission.file_name,
        )
            .execute(tx)
            .await?;

        Ok(())
    }

    async fn count_by_class_id(&self, conn: &mut DBConn, class_id: &str) -> Result<i64> {
        let submissions_count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM `submissions` WHERE `class_id` = ?",
            class_id
        )
        .fetch_one(conn)
        .await?;
        Ok(submissions_count)
    }

    async fn update_score_by_user_code_and_class_id<'c>(
        &self,
        tx: &mut TxConn,
        user_code: &str,
        class_id: &str,
        score: i64,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE `submissions` JOIN `users` ON `users`.`id` = `submissions`.`user_id` SET `score` = ? WHERE `users`.`code` = ? AND `class_id` = ?",
            score,
            user_code,
            class_id,
        )
            .execute(tx)
            .await?;

        Ok(())
    }

    async fn find_score_by_class_id_and_user_id(
        &self,
        conn: &mut DBConn,
        class_id: &str,
        user_id: &str,
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
            Some(score) => match score {
                None => Ok(None),
                Some(score) => Ok(Some(score)),
            },
        }
    }

    async fn find_all_by_class_id<'c>(
        &self,
        tx: &mut TxConn,
        class_id: &str,
    ) -> Result<Vec<Submission>> {
        let submissions: Vec<Submission> = sqlx::query_as(concat!(
        "SELECT `submissions`.`user_id`, `submissions`.`file_name`, `users`.`code` AS `user_code`",
        " FROM `submissions`",
        " JOIN `users` ON `users`.`id` = `submissions`.`user_id`",
        " WHERE `class_id` = ?",
        ))
        .bind(class_id)
        .fetch_all(tx)
        .await?;

        Ok(submissions)
    }
}
