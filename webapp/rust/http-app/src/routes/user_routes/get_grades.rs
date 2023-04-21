use actix_web::{web, HttpResponse};
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::summary::Summary;
use isucholar_core::services::class_service::{ClassService, HaveClassService};
use isucholar_core::services::registration_course_service::{
    HaveRegistrationCourseService, RegistrationCourseService,
};
use isucholar_core::services::user_service::{HaveUserService, UserService};
use isucholar_core::util;
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_http_core::responses::get_grade_response::GetGradeResponse;
use isucholar_http_core::routes::util::get_user_info;

// GET /api/users/me/grades 成績取得
pub async fn get_grades<
    Service: HaveClassService + HaveUserService + HaveRegistrationCourseService,
>(
    service: web::Data<Service>,
    session: actix_session::Session,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let registered_courses = service
        .registration_course_service()
        .find_courses_by_user_id(&user_id)
        .await?;

    // 科目毎の成績計算処理
    let mut course_results = Vec::with_capacity(registered_courses.len());
    let mut my_gpa = 0f64;
    let mut my_credits = 0;

    let class_service = service.class_service();

    for course in registered_courses {
        let course_result = class_service
            .get_user_course_result_by_course(&user_id, &course)
            .await?;
        let my_total_score = course_result.total_score;
        course_results.push(course_result);

        // 自分のGPA計算
        if course.status == CourseStatus::Closed {
            my_gpa += (my_total_score * course.credit as i64) as f64;
            my_credits += course.credit as i64;
        }
    }
    if my_credits > 0 {
        my_gpa = my_gpa / 100.0 / my_credits as f64;
    }

    let gpas = service.user_service().find_gpas_group_by_user_id().await?;

    Ok(HttpResponse::Ok().json(GetGradeResponse {
        course_results,
        summary: Summary {
            credits: my_credits,
            gpa: my_gpa,
            gpa_t_score: util::t_score_f64(my_gpa, &gpas),
            gpa_avg: util::average_f64(&gpas, 0.0),
            gpa_max: util::max_f64(&gpas, 0.0),
            gpa_min: util::min_f64(&gpas, 0.0),
        },
    }))
}
