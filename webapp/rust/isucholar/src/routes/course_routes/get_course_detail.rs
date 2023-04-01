use crate::responses::error::SqlxError;
use crate::responses::get_course_detail_response::GetCourseDetailResponse;
use actix_web::{web, HttpResponse};

// GET /api/courses/{course_id} 科目詳細の取得
pub async fn get_course_detail(
    pool: web::Data<sqlx::MySqlPool>,
    course_id: web::Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let course_id = &course_id.0;

    let res: Option<GetCourseDetailResponse> = sqlx::query_as(concat!(
        "SELECT `courses`.*, `users`.`name` AS `teacher`",
        " FROM `courses`",
        " JOIN `users` ON `courses`.`teacher_id` = `users`.`id`",
        " WHERE `courses`.`id` = ?",
    ))
    .bind(course_id)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(SqlxError)?;

    if let Some(res) = res {
        Ok(HttpResponse::Ok().json(res))
    } else {
        Err(actix_web::error::ErrorNotFound("No such course."))
    }
}
