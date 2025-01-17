use async_trait::async_trait;
use futures::StreamExt;
use isucholar_core::db::DBConn;
use isucholar_core::models::course::{Course, CourseCode, CourseID};
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use isucholar_core::models::user::UserID;
use isucholar_core::repos::error::Result;
use isucholar_core::repos::registration_course_repository::RegistrationCourseRepository;
use num_traits::ToPrimitive;

#[cfg(test)]
mod find_courses_by_user_id;
#[cfg(test)]
mod find_open_courses_by_user_id;
#[cfg(test)]
mod find_total_scores_by_course_id_group_by_user_id;

#[derive(Clone)]
pub struct RegistrationCourseRepositoryInfra {}

#[async_trait]
impl RegistrationCourseRepository for RegistrationCourseRepositoryInfra {
    async fn find_courses_by_user_id(
        &self,
        conn: &mut DBConn,
        user_id: &UserID,
    ) -> Result<Vec<Course>> {
        let registered_courses: Vec<Course> = sqlx::query_as!(
            Course,
            r"
                SELECT
                    courses.id as `id:CourseID`,
                    courses.code as `code:CourseCode`,
                    courses.type as `type_:CourseType`,
                    courses.name,
                    courses.description,
                    courses.credit,
                    courses.period,
                    courses.day_of_week as `day_of_week:DayOfWeek`,
                    courses.teacher_id as `teacher_id:UserID`,
                    courses.keywords,
                    courses.status as `status:CourseStatus`
                FROM `registrations`
                JOIN `courses` ON `registrations`.`course_id` = `courses`.`id`
                WHERE `user_id` = ?
            ",
            &user_id
        )
        .fetch_all(conn)
        .await?;

        Ok(registered_courses)
    }

    async fn find_open_courses_by_user_id(
        &self,
        conn: &mut DBConn,
        user_id: &UserID,
    ) -> Result<Vec<Course>> {
        let courses: Vec<Course> = sqlx::query_as!(
            Course,
            r"
                SELECT
                    courses.id as `id:CourseID`,
                    courses.code as `code:CourseCode`,
                    courses.type as `type_:CourseType`,
                    courses.name,
                    courses.description,
                    courses.credit,
                    courses.period,
                    courses.day_of_week as `day_of_week:DayOfWeek`,
                    courses.teacher_id as `teacher_id:UserID`,
                    courses.keywords,
                    courses.status as `status:CourseStatus`
                FROM `courses`
                JOIN `registrations` ON `courses`.`id` = `registrations`.`course_id`
                WHERE `courses`.`status` != ? AND `registrations`.`user_id` = ?
            ",
            CourseStatus::Closed,
            user_id
        )
        .fetch_all(conn)
        .await?;

        Ok(courses)
    }

    async fn find_total_scores_by_course_id_group_by_user_id(
        &self,
        conn: &mut DBConn,
        course_id: &CourseID,
    ) -> Result<Vec<i64>> {
        let mut rows = sqlx::query_scalar!(
        r"
                SELECT IFNULL(SUM(`submissions`.`score`), 0) AS `total_score`
                FROM `users`
                JOIN `registrations` ON `users`.`id` = `registrations`.`user_id`
                JOIN `courses` ON `registrations`.`course_id` = `courses`.`id`
                LEFT JOIN `classes` ON `courses`.`id` = `classes`.`course_id`
                LEFT JOIN `submissions` ON `users`.`id` = `submissions`.`user_id` AND `submissions`.`class_id` = `classes`.`id`
                WHERE `courses`.`id` = ?
                GROUP BY `users`.`id`
            ",
            course_id
        )
            .fetch(conn);
        let mut totals = Vec::new();
        while let Some(row) = rows.next().await {
            let total_score: sqlx::types::BigDecimal = row?;
            totals.push(total_score.to_i64().unwrap());
        }

        Ok(totals)
    }
}
