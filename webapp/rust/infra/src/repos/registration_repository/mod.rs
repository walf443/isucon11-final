use crate::db;
use async_trait::async_trait;
use isucholar_core::db::{DBConn, TxConn};
use isucholar_core::models::user::User;
use isucholar_core::models::user_type::UserType;
use isucholar_core::repos::error::Result;
use isucholar_core::repos::registration_repository::RegistrationRepository;

#[cfg(test)]
mod create_or_update;
#[cfg(test)]
mod exist_by_user_id_and_course_id;
#[cfg(test)]
mod find_users_by_course_id;

#[derive(Clone)]
pub struct RegistrationRepositoryInfra {}

#[async_trait]
impl RegistrationRepository for RegistrationRepositoryInfra {
    async fn create_or_update(
        &self,
        conn: &mut DBConn,
        user_id: &str,
        course_id: &str,
    ) -> Result<()> {
        sqlx::query!(
            "INSERT INTO `registrations` (`course_id`, `user_id`) VALUES (?, ?) ON DUPLICATE KEY UPDATE `course_id` = VALUES(`course_id`), `user_id` = VALUES(`user_id`)",
            course_id,
            user_id,
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    async fn exist_by_user_id_and_course_id<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        course_id: &str,
    ) -> Result<bool> {
        let registration_count: i64 = db::fetch_one_scalar(
            sqlx::query_scalar!(
                "SELECT COUNT(*) FROM `registrations` WHERE `user_id` = ? AND `course_id` = ?",
                user_id,
                course_id
            ),
            tx,
        )
        .await?;

        Ok(registration_count != 0)
    }

    async fn find_users_by_course_id(
        &self,
        conn: &mut DBConn,
        course_id: &str,
    ) -> Result<Vec<User>> {
        let users: Vec<User> = sqlx::query_as!(
            User,
            r"
                SELECT
                  `users`.id,
                  `users`.code,
                  `users`.name,
                  `users`.hashed_password,
                  `users`.type as `type_:UserType`
                FROM `users`
                JOIN `registrations` ON `users`.`id` = `registrations`.`user_id`
                WHERE `registrations`.`course_id` = ?
            ",
            course_id
        )
        .fetch_all(conn)
        .await?;

        Ok(users)
    }
}
