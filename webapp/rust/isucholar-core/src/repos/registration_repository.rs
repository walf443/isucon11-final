use crate::db;
use crate::db::TxConn;
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait RegistrationRepository {
    async fn exist_by_user_id_and_course_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        course_id: &str,
    ) -> Result<bool>;
}
pub struct RegistrationRepositoryImpl {}

#[async_trait]
impl RegistrationRepository for RegistrationRepositoryImpl {
    async fn exist_by_user_id_and_course_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        course_id: &str,
    ) -> Result<bool> {
        let registration_count: i64 = db::fetch_one_scalar(
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM `registrations` WHERE `user_id` = ? AND `course_id` = ?",
            )
            .bind(user_id)
            .bind(course_id),
            tx,
        )
        .await?;

        Ok(registration_count != 0)
    }
}
