use actix_web::{web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use isucholar_core::models::assignment_path::AssignmentPath;
use isucholar_core::models::class::ClassID;
use isucholar_core::models::course::CourseID;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::submission::CreateSubmission;
use isucholar_core::repos::class_repository::ClassRepository;
use isucholar_core::repos::course_repository::CourseRepository;
use isucholar_core::repos::registration_repository::RegistrationRepository;
use isucholar_core::repos::submission_repository::SubmissionRepository;
use isucholar_core::storages::submission_file_storage::SubmissionFileStorage;
use isucholar_http_core::responses::error::ResponseError::{
    ClassNotFound, CourseIsNotInProgress, CourseNotFound, InvalidFile, RegistrationAlready,
    SubmissionClosed,
};
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_http_core::routes::util::get_user_info;
use isucholar_infra::repos::class_repository::ClassRepositoryInfra;
use isucholar_infra::repos::course_repository::CourseRepositoryInfra;
use isucholar_infra::repos::registration_repository::RegistrationRepositoryInfra;
use isucholar_infra::repos::submission_repository::SubmissionRepositoryInfra;
use isucholar_infra::storages::submission_file_storage::SubmissionFileStorageInfra;

// POST /api/courses/{course_id}/classes/{class_id}/assignments 課題の提出
pub async fn submit_assignment(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    path: web::Path<AssignmentPath>,
    mut payload: actix_multipart::Multipart,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_id = CourseID::new(path.course_id.to_string());
    let class_id = ClassID::new(path.class_id.to_string());

    let mut tx = pool.begin().await?;
    let course_repo = CourseRepositoryInfra {};
    let status = course_repo
        .find_status_for_share_lock_by_id(&mut tx, &course_id)
        .await?;
    if let Some(status) = status {
        if status != CourseStatus::InProgress {
            return Err(CourseIsNotInProgress);
        }
    } else {
        return Err(CourseNotFound);
    }

    let registration_repo = RegistrationRepositoryInfra {};

    let is_registered = registration_repo
        .exist_by_user_id_and_course_id(&mut tx, &user_id, &course_id)
        .await?;
    if is_registered {
        return Err(RegistrationAlready);
    }

    let class_repo = ClassRepositoryInfra {};
    let submission_closed = class_repo
        .find_submission_closed_by_id_with_shared_lock(&mut tx, &class_id)
        .await?;

    if let Some(submission_closed) = submission_closed {
        if submission_closed {
            return Err(SubmissionClosed);
        }
    } else {
        return Err(ClassNotFound);
    }

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

    let submission_repo = SubmissionRepositoryInfra {};
    submission_repo
        .create_or_update(
            &mut tx,
            &CreateSubmission {
                user_id: user_id.clone(),
                class_id: class_id.clone(),
                file_name: file
                    .content_disposition()
                    .get_filename()
                    .unwrap()
                    .to_string(),
            },
        )
        .await?;

    let mut data = file
        .map_ok(|b| web::BytesMut::from(&b[..]))
        .try_concat()
        .await?;
    let submission_file_storage = SubmissionFileStorageInfra::new();
    submission_file_storage
        .upload(&class_id, &user_id, &mut data)
        .await?;

    tx.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}
