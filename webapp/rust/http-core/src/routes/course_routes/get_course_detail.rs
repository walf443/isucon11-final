use crate::responses::error::ResponseError::CourseNotFound;
use crate::responses::error::ResponseResult;
use actix_web::{web, HttpResponse};
use isucholar_core::models::course::CourseID;
use isucholar_core::services::course_service::{CourseService, HaveCourseService};

// GET /api/courses/{course_id} 科目詳細の取得
pub async fn get_course_detail<Service: HaveCourseService>(
    service: web::Data<Service>,
    course_id: web::Path<(String,)>,
) -> ResponseResult<HttpResponse> {
    let course_id = CourseID::new(course_id.0.to_string());

    let course_with_teacher = service
        .course_service()
        .find_with_teacher_by_id(&course_id)
        .await?;

    if let Some(c) = course_with_teacher {
        Ok(HttpResponse::Ok().json(c))
    } else {
        Err(CourseNotFound)
    }
}
