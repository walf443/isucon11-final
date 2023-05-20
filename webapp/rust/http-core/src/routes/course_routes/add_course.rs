use crate::responses::error::ResponseResult;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::models::course::{CourseCode, CourseID, CreateCourse};
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use isucholar_core::models::user::UserID;
use isucholar_core::services::course_service::{CourseService, HaveCourseService};
use isucholar_core::util;

#[derive(Debug, serde::Deserialize)]
pub struct AddCourseRequest {
    code: String,
    #[serde(rename = "type")]
    type_: CourseType,
    name: String,
    description: String,
    credit: u8,
    period: u8,
    day_of_week: DayOfWeek,
    keywords: String,
}

impl AddCourseRequest {
    fn convert_create_course(&self, course_id: String, user_id: UserID) -> CreateCourse {
        CreateCourse {
            id: CourseID::new(course_id),
            teacher_id: user_id,
            code: CourseCode::new(self.code.clone()),
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
    pub id: CourseID,
}

// POST /api/courses 新規科目登録
pub async fn add_course<Service: HaveCourseService>(
    service: web::Data<Service>,
    session: actix_session::Session,
    req: web::Json<AddCourseRequest>,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_id = util::new_ulid().await;
    let form = req.convert_create_course(course_id.clone(), user_id.clone());

    let course_id = service.course_service().create(&form).await?;

    Ok(HttpResponse::Created().json(AddCourseResponse { id: course_id }))
}
