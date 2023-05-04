use actix_web::web;
use isucholar_core::models::assignment_path::AssignmentPath;
use isucholar_core::repos::class_repository::ClassRepository;
use isucholar_core::repos::submission_repository::SubmissionRepository;
use isucholar_core::storages::submission_file_storage::SubmissionFileStorage;
use isucholar_http_core::responses::error::ResponseError::ClassNotFound;
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_infra::repos::class_repository::ClassRepositoryInfra;
use isucholar_infra::repos::submission_repository::SubmissionRepositoryInfra;
use isucholar_infra::storages::submission_file_storage::SubmissionFileStorageInfra;

// GET /api/courses/{course_id}/classes/{class_id}/assignments/export 提出済みの課題ファイルをzip形式で一括ダウンロード
pub async fn download_submitted_assignments(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<AssignmentPath>,
) -> ResponseResult<actix_files::NamedFile> {
    let class_id = path.class_id.clone();

    let mut tx = pool.begin().await?;
    let class_repo = ClassRepositoryInfra {};
    let is_exist = class_repo.for_update_by_id(&mut tx, &class_id).await?;

    if !is_exist {
        return Err(ClassNotFound);
    }
    let submission_repo = SubmissionRepositoryInfra {};
    let submissions = submission_repo
        .find_all_with_user_code_by_class_id(&mut tx, &class_id)
        .await?;

    let submission_file_storage = SubmissionFileStorageInfra::new();
    let zip_file_path = submission_file_storage
        .create_submissions_zip(&class_id, &submissions)
        .await?;

    class_repo
        .update_submission_closed_by_id(&mut tx, &class_id)
        .await?;

    tx.commit().await?;

    Ok(actix_files::NamedFile::open(&zip_file_path)?)
}
