use crate::database::DBPool;
use crate::models::course::Course;
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait RegistrationCourseRepository {
    async fn find_courses_by_user_id(&self, pool: &DBPool, user_id: &str) -> Result<Vec<Course>>;
}

pub struct RegistrationCourseRepositoryImpl {}

#[async_trait]
impl RegistrationCourseRepository for RegistrationCourseRepositoryImpl {
    async fn find_courses_by_user_id(&self, pool: &DBPool, user_id: &str) -> Result<Vec<Course>> {
        let registered_courses: Vec<Course> = sqlx::query_as(concat!(
            "SELECT `courses`.*",
            " FROM `registrations`",
            " JOIN `courses` ON `registrations`.`course_id` = `courses`.`id`",
            " WHERE `user_id` = ?",
        ))
        .bind(&user_id)
        .fetch_all(pool)
        .await?;

        Ok(registered_courses)
    }
}
