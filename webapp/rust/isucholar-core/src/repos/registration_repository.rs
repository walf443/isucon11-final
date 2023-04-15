use crate::db;
use crate::db::TxConn;
use crate::models::user::User;
use crate::repos::error::Result;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait RegistrationRepository {
    async fn create_or_update_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        course_id: &str,
    ) -> Result<()>;
    async fn exist_by_user_id_and_course_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        course_id: &str,
    ) -> Result<bool>;
    async fn find_users_by_course_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        course_id: &str,
    ) -> Result<Vec<User>>;
}

pub trait HaveRegistrationRepository {
    type Repo: RegistrationRepository + Sync;

    fn registration_repo(&self) -> &Self::Repo;
}

#[derive(Clone)]
pub struct RegistrationRepositoryImpl {}

#[async_trait]
impl RegistrationRepository for RegistrationRepositoryImpl {
    async fn create_or_update_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        course_id: &str,
    ) -> Result<()> {
        sqlx::query("INSERT INTO `registrations` (`course_id`, `user_id`) VALUES (?, ?) ON DUPLICATE KEY UPDATE `course_id` = VALUES(`course_id`), `user_id` = VALUES(`user_id`)")
            .bind(course_id)
            .bind(user_id)
            .execute(tx)
            .await?;

        Ok(())
    }

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

    async fn find_users_by_course_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        course_id: &str,
    ) -> Result<Vec<User>> {
        let users: Vec<User> = sqlx::query_as(concat!(
            "SELECT `users`.* FROM `users`",
            " JOIN `registrations` ON `users`.`id` = `registrations`.`user_id`",
            " WHERE `registrations`.`course_id` = ?",
        ))
        .bind(course_id)
        .fetch_all(tx)
        .await?;

        Ok(users)
    }
}
