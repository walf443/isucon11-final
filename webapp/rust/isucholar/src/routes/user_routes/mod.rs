pub mod get_grades;
pub mod get_me;
pub mod get_registered_courses;

use crate::requests::register_course_request::RegisterCourseRequestContent;
use crate::responses::error::SqlxError;
use crate::responses::get_grade_response::GetGradeResponse;
use crate::responses::get_registered_course_response::GetRegisteredCourseResponseContent;
use crate::responses::register_courses_error_response::RegisterCoursesErrorResponse;
use crate::routes::util::get_user_info;
use crate::{db, util};
use actix_web::{web, HttpResponse};
use futures::StreamExt;
use isucholar_core::models::class::Class;
use isucholar_core::models::class_score::ClassScore;
use isucholar_core::models::course::Course;
use isucholar_core::models::course_result::CourseResult;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::summary::Summary;
use isucholar_core::models::user::User;
use isucholar_core::models::user_type::UserType;
use num_traits::ToPrimitive;

// PUT /api/users/me/courses 履修登録
pub async fn register_courses(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    req: web::Json<Vec<RegisterCourseRequestContent>>,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let mut req = req.into_inner();
    req.sort_by(|x, y| x.id.cmp(&y.id));

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let mut errors = RegisterCoursesErrorResponse::default();
    let mut newly_added = Vec::new();
    for course_req in req {
        let course: Option<Course> = db::fetch_optional_as(
            sqlx::query_as("SELECT * FROM `courses` WHERE `id` = ? FOR SHARE").bind(&course_req.id),
            &mut tx,
        )
        .await
        .map_err(SqlxError)?;
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
        let count: i64 = db::fetch_one_scalar(
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM `registrations` WHERE `course_id` = ? AND `user_id` = ?",
            )
            .bind(&course.id)
            .bind(&user_id),
            &mut tx,
        )
        .await
        .map_err(SqlxError)?;
        if count > 0 {
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
    .await
    .map_err(SqlxError)?;

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
            .await
            .map_err(SqlxError)?;
    }

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::Ok().finish())
}
