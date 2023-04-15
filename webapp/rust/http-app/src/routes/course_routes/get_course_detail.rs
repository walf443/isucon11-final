use actix_web::{web, HttpResponse};
use isucholar_core::repos::course_repository::CourseRepository;
use isucholar_http_core::responses::error::ResponseError::CourseNotFound;
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_infra::repos::course_repository::CourseRepositoryImpl;

// GET /api/courses/{course_id} 科目詳細の取得
pub async fn get_course_detail(
    pool: web::Data<sqlx::MySqlPool>,
    course_id: web::Path<(String,)>,
) -> ResponseResult<HttpResponse> {
    let course_id = &course_id.0;

    let course_repo = CourseRepositoryImpl {};
    let course_with_teacher = course_repo
        .find_with_teacher_by_id(&pool, course_id)
        .await?;

    if let Some(c) = course_with_teacher {
        Ok(HttpResponse::Ok().json(c))
    } else {
        Err(CourseNotFound)
    }
}
