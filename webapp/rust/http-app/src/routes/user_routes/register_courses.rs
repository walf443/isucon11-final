use actix_web::{web, HttpResponse};
use isucholar_core::services::error::Error;
use isucholar_core::services::registration_course_service::{
    HaveRegistrationCourseService, RegistrationCourseService,
};
use isucholar_http_core::requests::register_course_request::RegisterCourseRequestContent;
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_http_core::routes::util::get_user_info;

// PUT /api/users/me/courses 履修登録
pub async fn register_courses<Service: HaveRegistrationCourseService>(
    service: web::Data<Service>,
    session: actix_session::Session,
    req: web::Json<Vec<RegisterCourseRequestContent>>,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let mut req = req.into_inner();
    req.sort_by(|x, y| x.id.cmp(&y.id));

    let course_ids = req.iter().map(|i| i.id.clone()).collect();

    let result = service
        .registration_course_service()
        .create(&user_id, &course_ids)
        .await;
    return match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => match e {
            Error::RegistrationCourseValidationError(errors) => {
                Ok(HttpResponse::BadRequest().json(errors))
            }
            _ => Err(e.into()),
        },
    };
}
