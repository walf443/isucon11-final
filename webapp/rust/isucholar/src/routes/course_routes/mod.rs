pub mod get_classes;
pub mod search_courses;

use crate::requests::search_courses_query::SearchCoursesQuery;
use crate::responses::error::SqlxError;
use crate::responses::get_course_detail_response::GetCourseDetailResponse;
use crate::routes::util::get_user_info;
use crate::{db, util};
use actix_web::{web, HttpResponse};
use isucholar_core::models::class::ClassWithSubmitted;
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use sqlx::Arguments;

const MYSQL_ERR_NUM_DUPLICATE_ENTRY: u16 = 1062;

#[derive(Debug, serde::Deserialize)]
pub struct AddCourseRequest {
    code: String,
    #[serde(rename = "type")]
    type_: CourseType,
    name: String,
    description: String,
    credit: i64,
    period: i64,
    day_of_week: DayOfWeek,
    keywords: String,
}

#[derive(Debug, serde::Serialize)]
pub struct AddCourseResponse {
    pub id: String,
}

// POST /api/courses 新規科目登録
pub async fn add_course(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    req: web::Json<AddCourseRequest>,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_id = util::new_ulid().await;
    let result = sqlx::query("INSERT INTO `courses` (`id`, `code`, `type`, `name`, `description`, `credit`, `period`, `day_of_week`, `teacher_id`, `keywords`) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&course_id)
        .bind(&req.code)
        .bind(&req.type_)
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.credit)
        .bind(&req.period)
        .bind(&req.day_of_week)
        .bind(&user_id)
        .bind(&req.keywords)
        .execute(pool.as_ref())
        .await;
    if let Err(sqlx::Error::Database(ref db_error)) = result {
        if let Some(mysql_error) = db_error.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>() {
            if mysql_error.number() == MYSQL_ERR_NUM_DUPLICATE_ENTRY {
                let course: Course = sqlx::query_as("SELECT * FROM `courses` WHERE `code` = ?")
                    .bind(&req.code)
                    .fetch_one(pool.as_ref())
                    .await
                    .map_err(SqlxError)?;
                if req.type_ != course.type_
                    || req.name != course.name
                    || req.description != course.description
                    || req.credit != course.credit as i64
                    || req.period != course.period as i64
                    || req.day_of_week != course.day_of_week
                    || req.keywords != course.keywords
                {
                    return Err(actix_web::error::ErrorConflict(
                        "A course with the same code already exists.",
                    ));
                } else {
                    return Ok(HttpResponse::Created().json(AddCourseResponse { id: course.id }));
                }
            }
        }
    }
    result.map_err(SqlxError)?;

    Ok(HttpResponse::Created().json(AddCourseResponse { id: course_id }))
}

// GET /api/courses/{course_id} 科目詳細の取得
pub async fn get_course_detail(
    pool: web::Data<sqlx::MySqlPool>,
    course_id: web::Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let course_id = &course_id.0;

    let res: Option<GetCourseDetailResponse> = sqlx::query_as(concat!(
        "SELECT `courses`.*, `users`.`name` AS `teacher`",
        " FROM `courses`",
        " JOIN `users` ON `courses`.`teacher_id` = `users`.`id`",
        " WHERE `courses`.`id` = ?",
    ))
    .bind(course_id)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(SqlxError)?;

    if let Some(res) = res {
        Ok(HttpResponse::Ok().json(res))
    } else {
        Err(actix_web::error::ErrorNotFound("No such course."))
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct SetCourseStatusRequest {
    status: CourseStatus,
}

// PUT /api/courses/{course_id}/status 科目のステータスを変更
pub async fn set_course_status(
    pool: web::Data<sqlx::MySqlPool>,
    course_id: web::Path<(String,)>,
    req: web::Json<SetCourseStatusRequest>,
) -> actix_web::Result<HttpResponse> {
    let course_id = &course_id.0;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let count: i64 = db::fetch_one_scalar(
        sqlx::query_scalar("SELECT COUNT(*) FROM `courses` WHERE `id` = ? FOR UPDATE")
            .bind(course_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if count == 0 {
        return Err(actix_web::error::ErrorNotFound("No such course."));
    }

    sqlx::query("UPDATE `courses` SET `status` = ? WHERE `id` = ?")
        .bind(&req.status)
        .bind(course_id)
        .execute(&mut tx)
        .await
        .map_err(SqlxError)?;

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::Ok().finish())
}
