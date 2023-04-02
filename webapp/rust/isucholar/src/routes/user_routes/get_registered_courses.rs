use crate::responses::error::ResponseResult;
use crate::responses::get_registered_course_response::GetRegisteredCourseResponseContent;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::repos::user_repository::{UserRepository, UserRepositoryImpl};

// GET /api/users/me/courses 履修中の科目一覧取得
pub async fn get_registered_courses(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let mut tx = pool.begin().await?;

    let courses: Vec<Course> = sqlx::query_as(concat!(
        "SELECT `courses`.*",
        " FROM `courses`",
        " JOIN `registrations` ON `courses`.`id` = `registrations`.`course_id`",
        " WHERE `courses`.`status` != ? AND `registrations`.`user_id` = ?",
    ))
    .bind(CourseStatus::Closed)
    .bind(&user_id)
    .fetch_all(&mut tx)
    .await?;

    // 履修科目が0件の時は空配列を返却
    let mut res = Vec::with_capacity(courses.len());
    let user_repo = UserRepositoryImpl {};
    for course in courses {
        let teacher = user_repo.find_in_tx(&mut tx, &course.teacher_id).await?;

        res.push(GetRegisteredCourseResponseContent {
            id: course.id,
            name: course.name,
            teacher: teacher.name,
            period: course.period,
            day_of_week: course.day_of_week,
        });
    }

    tx.commit().await?;

    Ok(HttpResponse::Ok().json(res))
}
