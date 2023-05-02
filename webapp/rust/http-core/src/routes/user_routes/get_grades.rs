use crate::responses::error::ResponseResult;
use crate::responses::get_grade_response::GetGradeResponse;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::models::user::UserID;
use isucholar_core::services::class_service::{ClassService, HaveClassService};
use isucholar_core::services::grade_summary_service::{
    GradeSummaryService, HaveGradeSummaryService,
};
use isucholar_core::services::registration_course_service::{
    HaveRegistrationCourseService, RegistrationCourseService,
};

// GET /api/users/me/grades 成績取得
pub async fn get_grades<
    Service: HaveClassService + HaveRegistrationCourseService + HaveGradeSummaryService,
>(
    service: web::Data<Service>,
    session: actix_session::Session,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;
    let user_id = UserID::new(user_id);

    let registered_courses = service
        .registration_course_service()
        .find_courses_by_user_id(&user_id.to_string())
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
