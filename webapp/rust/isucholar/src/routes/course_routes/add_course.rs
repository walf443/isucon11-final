use crate::responses::error::ResponseResult;
use crate::routes::util::get_user_info;
use crate::util;
use actix_web::{web, HttpResponse};
use isucholar_core::models::course::{Course, CreateCourse};
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
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
    let course_repo = CourseRepositoryImpl {};
    let form = CreateCourse {
        id: course_id,
        user_id: user_id.clone(),
        code: req.code.clone(),
        type_: req.type_.clone(),
        name: req.name.clone(),
        description: req.description.clone(),
        credit: req.credit.clone(),
        period: req.period.clone(),
        day_of_week: req.day_of_week.clone(),
        keywords: req.keywords.clone(),
    };
    let course_id = course_repo.create(&pool, &form).await?;

    Ok(HttpResponse::Created().json(AddCourseResponse { id: course_id }))
}
