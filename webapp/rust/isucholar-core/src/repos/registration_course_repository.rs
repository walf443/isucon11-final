use crate::database::DBPool;
use crate::models::course::Course;
use crate::repos::error::Result;
use async_trait::async_trait;
use futures::StreamExt;
use num_traits::ToPrimitive;

#[async_trait]
pub trait RegistrationCourseRepository {
    async fn find_courses_by_user_id(&self, pool: &DBPool, user_id: &str) -> Result<Vec<Course>>;
    async fn find_total_scores_by_course_id_group_by_user_id(
        &self,
        pool: &DBPool,
        course_id: &str,
    ) -> Result<Vec<i64>>;
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

    async fn find_total_scores_by_course_id_group_by_user_id(
        &self,
        pool: &DBPool,
        course_id: &str,
    ) -> Result<Vec<i64>> {
        let mut rows = sqlx::query_scalar(concat!(
        "SELECT IFNULL(SUM(`submissions`.`score`), 0) AS `total_score`",
        " FROM `users`",
        " JOIN `registrations` ON `users`.`id` = `registrations`.`user_id`",
        " JOIN `courses` ON `registrations`.`course_id` = `courses`.`id`",
        " LEFT JOIN `classes` ON `courses`.`id` = `classes`.`course_id`",
        " LEFT JOIN `submissions` ON `users`.`id` = `submissions`.`user_id` AND `submissions`.`class_id` = `classes`.`id`",
        " WHERE `courses`.`id` = ?",
        " GROUP BY `users`.`id`",
        ))
            .bind(course_id)
            .fetch(pool);
        let mut totals = Vec::new();
        while let Some(row) = rows.next().await {
            let total_score: sqlx::types::Decimal = row?;
            totals.push(total_score.to_i64().unwrap());
        }

        Ok(totals)
    }
}
