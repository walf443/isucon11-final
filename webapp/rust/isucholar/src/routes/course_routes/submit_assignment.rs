use crate::db;
use crate::responses::error::ResponseError::{
    ClassNotFound, CourseIsNotInProgress, CourseNotFound, InvalidFile, RegistrationAlready,
    SubmissionClosed,
};
use crate::responses::error::ResponseResult;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use isucholar_core::models::assignment_path::AssignmentPath;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::repos::course_repository::{CourseRepository, CourseRepositoryImpl};
use isucholar_core::ASSIGNMENTS_DIRECTORY;
use tokio::io::AsyncWriteExt;

// POST /api/courses/{course_id}/classes/{class_id}/assignments 課題の提出
pub async fn submit_assignment(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    path: web::Path<AssignmentPath>,
    mut payload: actix_multipart::Multipart,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_id = &path.course_id;
    let class_id = &path.class_id;

    let mut tx = pool.begin().await?;
    let course_repo = CourseRepositoryImpl {};
    let status = course_repo
        .find_status_for_share_lock_by_id_in_tx(&mut tx, course_id)
        .await?;
    if let Some(status) = status {
        if status != CourseStatus::InProgress {
            return Err(CourseIsNotInProgress);
        }
    } else {
        return Err(CourseNotFound);
    }

    let registration_count: i64 = db::fetch_one_scalar(
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM `registrations` WHERE `user_id` = ? AND `course_id` = ?",
        )
        .bind(&user_id)
        .bind(course_id),
        &mut tx,
    )
    .await?;
    if registration_count == 0 {
        return Err(RegistrationAlready);
    }

    let submission_closed: Option<bool> = db::fetch_optional_scalar(
        sqlx::query_scalar("SELECT `submission_closed` FROM `classes` WHERE `id` = ? FOR SHARE")
            .bind(class_id),
        &mut tx,
    )
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

    sqlx::query(
        "INSERT INTO `submissions` (`user_id`, `class_id`, `file_name`) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE `file_name` = VALUES(`file_name`)",
    )
        .bind(&user_id)
        .bind(class_id)
        .bind(file.content_disposition().get_filename())
        .execute(&mut tx)
        .await?;

    let mut data = file
        .map_ok(|b| web::BytesMut::from(&b[..]))
        .try_concat()
        .await?;
    let dst = format!("{}{}-{}.pdf", ASSIGNMENTS_DIRECTORY, class_id, user_id);
    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o666)
        .open(&dst)
        .await?;
    file.write_all_buf(&mut data).await?;

    tx.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}
