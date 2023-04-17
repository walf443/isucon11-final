use actix_web::{web, HttpResponse};
use isucholar_core::services::course_service::{CourseService, HaveCourseService};
use crate::responses::error::ResponseResult;
use crate::responses::get_registered_course_response::GetRegisteredCourseResponseContent;
use crate::routes::util::get_user_info;

// GET /api/users/me/courses 履修中の科目一覧取得
pub async fn get_registered_courses<Service: HaveCourseService>(
    service: web::Data<Service>,
    session: actix_session::Session,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_with_teachers = service
        .course_service()
        .find_open_courses_by_user_id(&user_id)
        .await?;

    // 履修科目が0件の時は空配列を返却
    let mut res = Vec::with_capacity(course_with_teachers.len());
    for (course, teacher) in course_with_teachers {
        res.push(GetRegisteredCourseResponseContent {
            id: course.id,
            name: course.name,
            teacher: teacher.name,
            period: course.period,
            day_of_week: course.day_of_week,
        });
    }

    Ok(HttpResponse::Ok().json(res))
}
