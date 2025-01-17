use crate::responses::error::ResponseError::{
    ClassNotFound, CourseIsNotInProgress, CourseNotFound, InvalidFile, RegistrationAlready,
    SubmissionClosed,
};
use crate::responses::error::ResponseResult;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use isucholar_core::models::assignment_path::AssignmentPath;
use isucholar_core::services::error::Error;
use isucholar_core::services::submission_service::{HaveSubmissionService, SubmissionService};

// POST /api/courses/{course_id}/classes/{class_id}/assignments 課題の提出
pub async fn submit_assignment<Service: HaveSubmissionService>(
    service: web::Data<Service>,
    session: actix_session::Session,
    path: web::Path<AssignmentPath>,
    mut payload: actix_multipart::Multipart,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_id = path.course_id.clone();
    let class_id = path.class_id.clone();

    let mut file = None;
    while let Some(field) = payload.next().await {
        let field = field.map_err(|_| InvalidFile)?;
        let content_disposition = field.content_disposition();
        if let Some(name) = content_disposition.get_name() {
            if name == "file" {
                file = Some(field);
                break;
            }
        }
    }
    if file.is_none() {
        return Err(InvalidFile);
    }
    let file = file.unwrap();

    let file_name = file
        .content_disposition()
        .get_filename()
        .unwrap()
        .to_string();

    let mut data = file
        .map_ok(|b| web::BytesMut::from(&b[..]))
        .try_concat()
        .await?;

    let result = service
        .submission_service()
        .create_or_update(&user_id, &course_id, &class_id, &file_name, &mut data)
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => match e {
            Error::CourseIsNotInProgress => Err(CourseIsNotInProgress),
            Error::CourseNotFound => Err(CourseNotFound),
            Error::RegistrationAlready => Err(RegistrationAlready),
            Error::SubmissionClosed => Err(SubmissionClosed),
            Error::ClassNotFound => Err(ClassNotFound),
            _ => Err(e.into()),
        },
    }
}
