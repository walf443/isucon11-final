use crate::db;
use crate::responses::error::SqlxError;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use isucholar_core::models::assignment_path::AssignmentPath;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::ASSIGNMENTS_DIRECTORY;
use tokio::io::AsyncWriteExt;

// POST /api/courses/{course_id}/classes/{class_id}/assignments 課題の提出
pub async fn submit_assignment(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    path: web::Path<AssignmentPath>,
    mut payload: actix_multipart::Multipart,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_id = &path.course_id;
    let class_id = &path.class_id;

    let mut tx = pool.begin().await.map_err(SqlxError)?;
    let status: Option<CourseStatus> = db::fetch_optional_scalar(
        sqlx::query_scalar("SELECT `status` FROM `courses` WHERE `id` = ? FOR SHARE")
            .bind(course_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if let Some(status) = status {
        if status != CourseStatus::InProgress {
            return Err(actix_web::error::ErrorBadRequest(
                "This course is not in progress.",
            ));
        }
    } else {
        return Err(actix_web::error::ErrorNotFound("No such course."));
    }

    let registration_count: i64 = db::fetch_one_scalar(
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM `registrations` WHERE `user_id` = ? AND `course_id` = ?",
        )
        .bind(&user_id)
        .bind(course_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if registration_count == 0 {
        return Err(actix_web::error::ErrorBadRequest(
            "You have not taken this course.",
        ));
    }

    let submission_closed: Option<bool> = db::fetch_optional_scalar(
        sqlx::query_scalar("SELECT `submission_closed` FROM `classes` WHERE `id` = ? FOR SHARE")
            .bind(class_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if let Some(submission_closed) = submission_closed {
        if submission_closed {
            return Err(actix_web::error::ErrorBadRequest(
                "Submission has been closed for this class.",
            ));
        }
    } else {
        return Err(actix_web::error::ErrorNotFound("No such class."));
    }

    let mut file = None;
    while let Some(field) = payload.next().await {
        let field = field.map_err(|_| actix_web::error::ErrorBadRequest("Invalid file."))?;
        if let content_disposition = field.content_disposition() {
            if let Some(name) = content_disposition.get_name() {
                if name == "file" {
                    file = Some(field);
                    break;
                }
            }
        }
    }
    if file.is_none() {
        return Err(actix_web::error::ErrorBadRequest("Invalid file."));
    }
    let file = file.unwrap();

    sqlx::query(
        "INSERT INTO `submissions` (`user_id`, `class_id`, `file_name`) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE `file_name` = VALUES(`file_name`)",
    )
        .bind(&user_id)
        .bind(class_id)
        .bind(file.content_disposition().get_filename())
        .execute(&mut tx)
        .await
        .map_err(SqlxError)?;

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

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::NoContent().finish())
}
