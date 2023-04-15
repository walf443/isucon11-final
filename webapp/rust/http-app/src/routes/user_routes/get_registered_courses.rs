use crate::responses::error::ResponseResult;
use crate::responses::get_registered_course_response::GetRegisteredCourseResponseContent;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::repos::registration_course_repository::RegistrationCourseRepository;
use isucholar_core::repos::user_repository::UserRepository;
use isucholar_infra::repos::registration_course_repository::RegistrationCourseRepositoryImpl;
use isucholar_infra::repos::user_repository::UserRepositoryImpl;

// GET /api/users/me/courses 履修中の科目一覧取得
pub async fn get_registered_courses(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let mut tx = pool.begin().await?;
    let registration_course_repo = RegistrationCourseRepositoryImpl {};
    let courses = registration_course_repo
        .find_open_courses_by_user_id_in_tx(&mut tx, &user_id)
        .await?;

    // 履修科目が0件の時は空配列を返却
    let mut res = Vec::with_capacity(courses.len());
    let user_repo = UserRepositoryImpl {};
    for course in courses {
        let teacher = user_repo.find_in_tx(&mut tx, &course.teacher_id).await?;

        res.push(GetRegisteredCourseResponseContent {
            id: course.id,
            name: course.name,
            teacher: teacher.name,
            period: course.period,
            day_of_week: course.day_of_week,
        });
    }

    tx.commit().await?;

    Ok(HttpResponse::Ok().json(res))
}
