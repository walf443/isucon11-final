use crate::db::{DBPool, TxConn};
use crate::models::submission::{CreateSubmission, Submission};
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SubmissionRepository {
    async fn create_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        submission: &CreateSubmission,
    ) -> Result<()>;
    async fn count_by_class_id(&self, pool: &DBPool, class_id: &str) -> Result<i64>;
    async fn update_score_by_user_code_and_class_id<'c>(
        &self,
        tx: &mut TxConn,
        user_code: &str,
        class_id: &str,
        score: i64,
    ) -> Result<()>;
    async fn find_score_by_class_id_and_user_id(
        &self,
        pool: &DBPool,
        class_id: &str,
        user_id: &str,
    ) -> Result<Option<Option<u8>>>;
    async fn find_all_by_class_id_in_tx<'c>(
        &self,
        tx: &mut TxConn,
        class_id: &str,
    ) -> Result<Vec<Submission>>;
}

pub struct SubmissionRepositoryImpl {}

#[async_trait]
impl SubmissionRepository for SubmissionRepositoryImpl {
    async fn create_in_tx<'c>(&self, tx: &mut TxConn, submission: &CreateSubmission) -> Result<()> {
        sqlx::query(
            "INSERT INTO `submissions` (`user_id`, `class_id`, `file_name`) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE `file_name` = VALUES(`file_name`)",
        )
            .bind(&submission.user_id)
            .bind(&submission.class_id)
            .bind(&submission.file_name)
            .execute(tx)
            .await?;

        Ok(())
    }

    async fn count_by_class_id(&self, pool: &DBPool, class_id: &str) -> Result<i64> {
        let submissions_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM `submissions` WHERE `class_id` = ?")
                .bind(class_id)
                .fetch_one(pool)
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
        sqlx::query("UPDATE `submissions` JOIN `users` ON `users`.`id` = `submissions`.`user_id` SET `score` = ? WHERE `users`.`code` = ? AND `class_id` = ?")
            .bind(score)
            .bind(user_code)
            .bind(class_id)
            .execute(tx)
            .await?;

        Ok(())
    }

    async fn find_score_by_class_id_and_user_id(
        &self,
        pool: &DBPool,
        class_id: &str,
        user_id: &str,
    ) -> Result<Option<Option<u8>>> {
        let score: Option<Option<u8>> = sqlx::query_scalar(concat!(
            "SELECT `submissions`.`score` FROM `submissions`",
            " WHERE `user_id` = ? AND `class_id` = ?"
        ))
        .bind(user_id)
        .bind(class_id)
        .fetch_optional(pool)
        .await?;

        Ok(score)
    }

    async fn find_all_by_class_id_in_tx<'c>(
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
