use actix_web::{HttpResponse, web};
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::user::User;
use crate::db;
use crate::responses::error::SqlxError;
use crate::responses::get_registered_course_response::GetRegisteredCourseResponseContent;
use crate::routes::util::get_user_info;

// GET /api/users/me/courses 履修中の科目一覧取得
pub async fn get_registered_courses(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let courses: Vec<Course> = sqlx::query_as(concat!(
    "SELECT `courses`.*",
    " FROM `courses`",
    " JOIN `registrations` ON `courses`.`id` = `registrations`.`course_id`",
    " WHERE `courses`.`status` != ? AND `registrations`.`user_id` = ?",
    ))
        .bind(CourseStatus::Closed)
        .bind(&user_id)
        .fetch_all(&mut tx)
        .await
        .map_err(SqlxError)?;

    // 履修科目が0件の時は空配列を返却
    let mut res = Vec::with_capacity(courses.len());
    for course in courses {
        let teacher: User = db::fetch_one_as(
            sqlx::query_as("SELECT * FROM `users` WHERE `id` = ?").bind(&course.teacher_id),
            &mut tx,
        )
            .await
            .map_err(SqlxError)?;

        res.push(GetRegisteredCourseResponseContent {
            id: course.id,
            name: course.name,
            teacher: teacher.name,
            period: course.period,
            day_of_week: course.day_of_week,
        });
    }

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::Ok().json(res))
}
