pub mod add_course;
pub mod get_classes;
pub mod search_courses;

use crate::requests::search_courses_query::SearchCoursesQuery;
use crate::responses::error::SqlxError;
use crate::responses::get_course_detail_response::GetCourseDetailResponse;
use crate::routes::util::get_user_info;
use crate::{db, util};
use actix_web::{web, HttpResponse};
use isucholar_core::models::class::ClassWithSubmitted;
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use sqlx::Arguments;

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

#[derive(Debug, serde::Deserialize)]
pub struct SetCourseStatusRequest {
    status: CourseStatus,
}

// PUT /api/courses/{course_id}/status 科目のステータスを変更
pub async fn set_course_status(
    pool: web::Data<sqlx::MySqlPool>,
    course_id: web::Path<(String,)>,
    req: web::Json<SetCourseStatusRequest>,
) -> actix_web::Result<HttpResponse> {
    let course_id = &course_id.0;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let count: i64 = db::fetch_one_scalar(
        sqlx::query_scalar("SELECT COUNT(*) FROM `courses` WHERE `id` = ? FOR UPDATE")
            .bind(course_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if count == 0 {
        return Err(actix_web::error::ErrorNotFound("No such course."));
    }

    sqlx::query("UPDATE `courses` SET `status` = ? WHERE `id` = ?")
        .bind(&req.status)
        .bind(course_id)
        .execute(&mut tx)
        .await
        .map_err(SqlxError)?;

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::Ok().finish())
}
