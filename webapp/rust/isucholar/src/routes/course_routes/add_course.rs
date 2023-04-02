use crate::responses::error::{ResponseError, ResponseResult};
use crate::routes::util::get_user_info;
use crate::util;
use actix_web::{web, HttpResponse};
use isucholar_core::models::course::Course;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use isucholar_core::MYSQL_ERR_NUM_DUPLICATE_ENTRY;
use isucholar_core::repos::course_repository::{CourseRepository, CourseRepositoryImpl};

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
) -> ResponseResult<HttpResponse> {
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

    let course_repo = CourseRepositoryImpl {};
    if let Err(sqlx::Error::Database(ref db_error)) = result {
        if let Some(mysql_error) = db_error.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>() {
            if mysql_error.number() == MYSQL_ERR_NUM_DUPLICATE_ENTRY {
                let course = course_repo.find_by_code(&pool, &req.code).await?;

                if req.type_ != course.type_
                    || req.name != course.name
                    || req.description != course.description
                    || req.credit != course.credit as i64
                    || req.period != course.period as i64
                    || req.day_of_week != course.day_of_week
                    || req.keywords != course.keywords
                {
                    return Err(ResponseError::ActixError(actix_web::error::ErrorConflict(
                        "A course with the same code already exists.",
                    )));
                } else {
                    return Ok(HttpResponse::Created().json(AddCourseResponse { id: course.id }));
                }
            }
        }
    }
    result?;

    Ok(HttpResponse::Created().json(AddCourseResponse { id: course_id }))
}
