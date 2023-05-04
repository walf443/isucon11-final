use crate::responses::error::ResponseError::ClassNotFound;
use crate::responses::error::ResponseResult;
use actix_web::web;
use isucholar_core::models::assignment_path::AssignmentPath;
use isucholar_core::services::error::Error;
use isucholar_core::services::submission_service::{HaveSubmissionService, SubmissionService};

// GET /api/courses/{course_id}/classes/{class_id}/assignments/export 提出済みの課題ファイルをzip形式で一括ダウンロード
pub async fn download_submitted_assignments<Service: HaveSubmissionService>(
    service: web::Data<Service>,
    path: web::Path<AssignmentPath>,
) -> ResponseResult<actix_files::NamedFile> {
    let class_id = path.class_id.clone();

    let result = service
        .submission_service()
        .download_submissions_zip(&class_id)
        .await;
    match result {
        Ok(zip_file_path) => Ok(actix_files::NamedFile::open(&zip_file_path)?),
        Err(e) => match e {
            Error::ClassNotFound => Err(ClassNotFound),
            _ => Err(e.into()),
        },
    }
}
