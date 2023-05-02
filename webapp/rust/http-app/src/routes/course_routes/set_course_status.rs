use actix_web::{web, HttpResponse};
use isucholar_core::models::course::CourseID;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::services::course_service::{CourseService, HaveCourseService};
use isucholar_http_core::responses::error::ResponseResult;

#[derive(Debug, serde::Deserialize)]
pub struct SetCourseStatusRequest {
    status: CourseStatus,
}

// PUT /api/courses/{course_id}/status 科目のステータスを変更
pub async fn set_course_status<Service: HaveCourseService>(
    service: web::Data<Service>,
    course_id: web::Path<(String,)>,
    req: web::Json<SetCourseStatusRequest>,
) -> ResponseResult<HttpResponse> {
    let course_id = CourseID::new(course_id.0.to_string());

    service
        .course_service()
        .update_status_by_id(&course_id, &req.status)
        .await?;

    Ok(HttpResponse::Ok().finish())
}
