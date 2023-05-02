use async_trait::async_trait;
use isucholar_core::db::DBConn;
use isucholar_core::models::course::CourseID;
use isucholar_core::models::user::{User, UserCode, UserID};
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
        user_id: &UserID,
        course_id: &CourseID,
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

    async fn exist_by_user_id_and_course_id(
        &self,
        conn: &mut DBConn,
        user_id: &UserID,
        course_id: &CourseID,
    ) -> Result<bool> {
        let registration_count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM `registrations` WHERE `user_id` = ? AND `course_id` = ?",
            user_id,
            course_id
        )
        .fetch_one(conn)
        .await?;

        Ok(registration_count != 0)
    }

    async fn find_users_by_course_id(
        &self,
        conn: &mut DBConn,
        course_id: &CourseID,
    ) -> Result<Vec<User>> {
        let users: Vec<User> = sqlx::query_as!(
            User,
            r"
                SELECT
                  `users`.id as `id:UserID`,
                  `users`.code as `code:UserCode`,
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
