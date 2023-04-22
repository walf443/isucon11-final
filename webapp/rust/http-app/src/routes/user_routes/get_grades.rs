use actix_web::{web, HttpResponse};
use isucholar_core::services::class_service::{ClassService, HaveClassService};
use isucholar_core::services::grade_summary_service::{
    GradeSummaryService, HaveGradeSummaryService,
};
use isucholar_core::services::registration_course_service::{
    HaveRegistrationCourseService, RegistrationCourseService,
};
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_http_core::responses::get_grade_response::GetGradeResponse;
use isucholar_http_core::routes::util::get_user_info;

// GET /api/users/me/grades 成績取得
pub async fn get_grades<
    Service: HaveClassService + HaveRegistrationCourseService + HaveGradeSummaryService,
>(
    service: web::Data<Service>,
    session: actix_session::Session,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let registered_courses = service
        .registration_course_service()
        .find_courses_by_user_id(&user_id)
        .await?;

    let (course_results, my_gpa, my_credits) = service
        .class_service()
        .get_user_courses_result_by_courses(&user_id, &registered_courses)
        .await?;

    let summary = service
        .grade_summary_service()
        .get_summary_by_user_gpa(my_gpa, my_credits)
        .await?;

    Ok(HttpResponse::Ok().json(GetGradeResponse {
        course_results,
        summary,
    }))
}
