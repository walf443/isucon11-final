use actix_web::{web, HttpResponse};
use isucholar_core::models::course::CreateCourse;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use isucholar_core::repos::course_repository::CourseRepository;
use isucholar_core::util;
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_http_core::routes::util::get_user_info;
use isucholar_infra::repos::course_repository::CourseRepositoryInfra;

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

impl AddCourseRequest {
    fn convert_create_course(&self, course_id: String, user_id: String) -> CreateCourse {
        CreateCourse {
            id: course_id,
            user_id: user_id,
            code: self.code.clone(),
            type_: self.type_.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            credit: self.credit.clone(),
            period: self.period.clone(),
            day_of_week: self.day_of_week.clone(),
            keywords: self.keywords.clone(),
        }
    }
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
    let course_repo = CourseRepositoryInfra {};
    let form = req.convert_create_course(course_id.clone(), user_id.clone());
    let course_id = course_repo.create(&pool, &form).await?;

    Ok(HttpResponse::Created().json(AddCourseResponse { id: course_id }))
}
