use actix_web::{web, HttpResponse};
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::repos::course_repository::CourseRepository;
use isucholar_http_core::responses::error::ResponseError::CourseNotFound;
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_infra::repos::course_repository::CourseRepositoryInfra;

#[derive(Debug, serde::Deserialize)]
pub struct SetCourseStatusRequest {
    status: CourseStatus,
}

// PUT /api/courses/{course_id}/status 科目のステータスを変更
pub async fn set_course_status(
    pool: web::Data<sqlx::MySqlPool>,
    course_id: web::Path<(String,)>,
    req: web::Json<SetCourseStatusRequest>,
) -> ResponseResult<HttpResponse> {
    let course_id = &course_id.0;

    let mut tx = pool.begin().await?;
    let course_repo = CourseRepositoryInfra {};
    let is_exist = course_repo.for_update_by_id(&mut tx, course_id).await?;
    if !is_exist {
        return Err(CourseNotFound);
    }

    course_repo
        .update_status_by_id(&mut tx, course_id, &req.status)
        .await?;

    tx.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
