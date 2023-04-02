use crate::requests::register_course_request::RegisterCourseRequestContent;
use crate::responses::error::ResponseResult;
use crate::responses::register_courses_error_response::RegisterCoursesErrorResponse;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::repos::course_repository::{CourseRepository, CourseRepositoryImpl};
use isucholar_core::repos::registration_repository::{
    RegistrationRepository, RegistrationRepositoryImpl,
};

// PUT /api/users/me/courses 履修登録
pub async fn register_courses(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    req: web::Json<Vec<RegisterCourseRequestContent>>,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let mut req = req.into_inner();
    req.sort_by(|x, y| x.id.cmp(&y.id));

    let mut tx = pool.begin().await?;

    let mut errors = RegisterCoursesErrorResponse::default();
    let mut newly_added = Vec::new();
    let course_repo = CourseRepositoryImpl {};
    let registration_repo = RegistrationRepositoryImpl {};
    for course_req in req {
        let course = course_repo
            .find_for_share_lock_by_id_in_tx(&mut tx, &course_req.id)
            .await?;
        if course.is_none() {
            errors.course_not_found.push(course_req.id);
            continue;
        }
        let course = course.unwrap();

        if course.status != CourseStatus::Registration {
            errors.not_registrable_status.push(course.id);
            continue;
        }

        // すでに履修登録済みの科目は無視する
        let is_exist = registration_repo
            .exist_by_user_id_and_course_id_in_tx(&mut tx, &user_id, &course.id)
            .await?;
        if is_exist {
            continue;
        }

        newly_added.push(course);
    }

    let already_registered: Vec<Course> = sqlx::query_as(concat!(
        "SELECT `courses`.*",
        " FROM `courses`",
        " JOIN `registrations` ON `courses`.`id` = `registrations`.`course_id`",
        " WHERE `courses`.`status` != ? AND `registrations`.`user_id` = ?",
    ))
    .bind(CourseStatus::Closed)
    .bind(&user_id)
    .fetch_all(&mut tx)
    .await?;

    for course1 in &newly_added {
        for course2 in already_registered.iter().chain(newly_added.iter()) {
            if course1.id != course2.id
                && course1.period == course2.period
                && course1.day_of_week == course2.day_of_week
            {
                errors.schedule_conflict.push(course1.id.to_owned());
                break;
            }
        }
    }

    if !errors.course_not_found.is_empty()
        || !errors.not_registrable_status.is_empty()
        || !errors.schedule_conflict.is_empty()
    {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    for course in newly_added {
        sqlx::query("INSERT INTO `registrations` (`course_id`, `user_id`) VALUES (?, ?) ON DUPLICATE KEY UPDATE `course_id` = VALUES(`course_id`), `user_id` = VALUES(`user_id`)")
            .bind(course.id)
            .bind(&user_id)
            .execute(&mut tx)
            .await?;
    }

    tx.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
