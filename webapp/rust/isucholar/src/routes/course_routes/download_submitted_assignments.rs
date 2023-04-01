use crate::db;
use crate::responses::error::SqlxError;
use actix_web::web;
use isucholar_core::models::assignment_path::AssignmentPath;
use isucholar_core::models::submission::Submission;
use isucholar_core::ASSIGNMENTS_DIRECTORY;

// GET /api/courses/{course_id}/classes/{class_id}/assignments/export 提出済みの課題ファイルをzip形式で一括ダウンロード
pub async fn download_submitted_assignments(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<AssignmentPath>,
) -> actix_web::Result<actix_files::NamedFile> {
    let class_id = &path.class_id;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let class_count: i64 = db::fetch_one_scalar(
        sqlx::query_scalar("SELECT COUNT(*) FROM `classes` WHERE `id` = ? FOR UPDATE")
            .bind(class_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if class_count == 0 {
        return Err(actix_web::error::ErrorNotFound("No such class."));
    }
    let submissions: Vec<Submission> = sqlx::query_as(concat!(
        "SELECT `submissions`.`user_id`, `submissions`.`file_name`, `users`.`code` AS `user_code`",
        " FROM `submissions`",
        " JOIN `users` ON `users`.`id` = `submissions`.`user_id`",
        " WHERE `class_id` = ?",
    ))
    .bind(class_id)
    .fetch_all(&mut tx)
    .await
    .map_err(SqlxError)?;

    let zip_file_path = format!("{}{}.zip", ASSIGNMENTS_DIRECTORY, class_id);
    create_submissions_zip(&zip_file_path, class_id, &submissions).await?;

    sqlx::query("UPDATE `classes` SET `submission_closed` = true WHERE `id` = ?")
        .bind(class_id)
        .execute(&mut tx)
        .await
        .map_err(SqlxError)?;

    tx.commit().await.map_err(SqlxError)?;

    Ok(actix_files::NamedFile::open(&zip_file_path)?)
}

async fn create_submissions_zip(
    zip_file_path: &str,
    class_id: &str,
    submissions: &[Submission],
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
                ASSIGNMENTS_DIRECTORY, class_id, submission.user_id
            ))
            .arg(&format!(
                "{}{}-{}",
                tmp_dir, submission.user_code, submission.file_name
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