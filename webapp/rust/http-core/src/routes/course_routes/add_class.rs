use crate::responses::error::ResponseError::{
    CourseConflict, CourseIsNotInProgress, CourseNotFound,
};
use crate::responses::error::ResponseResult;
use actix_web::{web, HttpResponse};
use isucholar_core::models::class::{ClassID, CreateClass};
use isucholar_core::models::course::CourseID;
use isucholar_core::services::class_service::{ClassService, HaveClassService};
use isucholar_core::services::error::Error;

#[derive(Debug, serde::Deserialize)]
pub struct AddClassRequest {
    part: u8,
    title: String,
    description: String,
}

#[derive(Debug, serde::Serialize)]
struct AddClassResponse {
    class_id: ClassID,
}

// POST /api/courses/{course_id}/classes 新規講義(&課題)追加
pub async fn add_class<Service: HaveClassService>(
    service: web::Data<Service>,
    course_id: web::Path<(String,)>,
    req: web::Json<AddClassRequest>,
) -> ResponseResult<HttpResponse> {
    let course_id = CourseID::new(course_id.0.to_string());

    let form = CreateClass {
        course_id: course_id.clone(),
        part: req.part,
        title: req.title.clone(),
        description: req.description.clone(),
    };

    let result = service.class_service().create(&form).await;

    match result {
        Ok(class_id) => Ok(HttpResponse::Created().json(AddClassResponse { class_id })),
        Err(e) => match e {
            Error::CourseNotFound => Err(CourseNotFound),
            Error::CourseIsNotInProgress => Err(CourseIsNotInProgress),
            Error::CourseConflict => Err(CourseConflict),
            _ => Err(e.into()),
        },
    }
}
