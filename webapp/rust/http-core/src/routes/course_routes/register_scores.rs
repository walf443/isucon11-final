use crate::responses::error::ResponseError::{ClassIsNotSubmissionClosed, ClassNotFound};
use crate::responses::error::ResponseResult;
use actix_web::{web, HttpResponse};
use isucholar_core::models::assignment_path::AssignmentPath;
use isucholar_core::models::class::ClassID;
use isucholar_core::models::score::Score;
use isucholar_core::services::error::Error;
use isucholar_core::services::submission_service::{HaveSubmissionService, SubmissionService};

// PUT /api/courses/{course_id}/classes/{class_id}/assignments/scores 採点結果登録
pub async fn register_scores<Service: HaveSubmissionService>(
    service: web::Data<Service>,
    path: web::Path<AssignmentPath>,
    req: web::Json<Vec<Score>>,
) -> ResponseResult<HttpResponse> {
    let class_id = ClassID::new(path.class_id.to_string());

    let result = service
        .submission_service()
        .update_user_scores_by_class_id(&class_id, &req.into_inner())
        .await;
    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => match e {
            Error::ClassNotFound => Err(ClassNotFound),
            Error::ClassIsNotSubmissionClosed => Err(ClassIsNotSubmissionClosed),
            _ => Err(e.into()),
        },
    }
}
