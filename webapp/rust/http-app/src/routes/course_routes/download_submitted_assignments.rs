use actix_web::web;
use isucholar_core::models::assignment_path::AssignmentPath;
use isucholar_core::models::submission::SubmissionWithUserCode;
use isucholar_core::repos::class_repository::ClassRepository;
use isucholar_core::repos::submission_repository::SubmissionRepository;
use isucholar_core::ASSIGNMENTS_DIRECTORY;
use isucholar_http_core::responses::error::ResponseError::ClassNotFound;
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_infra::repos::class_repository::ClassRepositoryInfra;
use isucholar_infra::repos::submission_repository::SubmissionRepositoryInfra;

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

    let zip_file_path = format!("{}{}.zip", ASSIGNMENTS_DIRECTORY, class_id.to_string());
    create_submissions_zip(&zip_file_path, &class_id.to_string(), &submissions).await?;

    class_repo
        .update_submission_closed_by_id(&mut tx, &class_id)
        .await?;

    tx.commit().await?;

    Ok(actix_files::NamedFile::open(&zip_file_path)?)
}

async fn create_submissions_zip(
    zip_file_path: &str,
    class_id: &str,
    submissions: &[SubmissionWithUserCode],
) -> std::io::Result<()> {
    let tmp_dir = format!("{}{}/", ASSIGNMENTS_DIRECTORY, class_id);
    tokio::process::Command::new("rm")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .arg("-rf")
        .arg(&tmp_dir)
        .status()
        .await?;
    tokio::process::Command::new("mkdir")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .arg(&tmp_dir)
        .status()
        .await?;

    // ファイル名を指定の形式に変更
    for submission in submissions {
        tokio::process::Command::new("cp")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .arg(&format!(
                "{}{}-{}.pdf",
                ASSIGNMENTS_DIRECTORY,
                class_id,
                submission.user_id.to_string()
            ))
            .arg(&format!(
                "{}{}-{}",
                tmp_dir,
                submission.user_code.to_string(),
                submission.file_name
            ))
            .status()
            .await?;
    }

    // -i 'tmp_dir/*': 空zipを許す
    tokio::process::Command::new("zip")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .arg("-j")
        .arg("-r")
        .arg(zip_file_path)
        .arg(&tmp_dir)
        .arg("-i")
        .arg(&format!("{}*", tmp_dir))
        .status()
        .await?;
    Ok(())
}
