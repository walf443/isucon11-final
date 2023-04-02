use crate::database::DBPool;
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SubmissionRepository {
    async fn count_by_class_id(&self, pool: &DBPool, class_id: &str) -> Result<i64>;
    async fn find_scores_by_class_id_and_user_id(
        &self,
        pool: &DBPool,
        class_id: &str,
        user_id: &str,
    ) -> Result<Option<Option<u8>>>;
}

pub struct SubmissionRepositoryImpl {}

#[async_trait]
impl SubmissionRepository for SubmissionRepositoryImpl {
    async fn count_by_class_id(&self, pool: &DBPool, class_id: &str) -> Result<i64> {
        let submissions_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM `submissions` WHERE `class_id` = ?")
                .bind(class_id)
                .fetch_one(pool)
                .await?;
        Ok(submissions_count)
    }

    async fn find_scores_by_class_id_and_user_id(
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
}
