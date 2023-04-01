use crate::db;
use crate::responses::error::SqlxError;
use actix_web::{web, HttpResponse};
use isucholar_core::models::assignment_path::AssignmentPath;
use isucholar_core::models::score::Score;

// PUT /api/courses/{course_id}/classes/{class_id}/assignments/scores 採点結果登録
pub async fn register_scores(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<AssignmentPath>,
    req: web::Json<Vec<Score>>,
) -> actix_web::Result<HttpResponse> {
    let class_id = &path.class_id;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let submission_closed: Option<bool> = db::fetch_optional_scalar(
        sqlx::query_scalar("SELECT `submission_closed` FROM `classes` WHERE `id` = ? FOR SHARE")
            .bind(class_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if let Some(submission_closed) = submission_closed {
        if !submission_closed {
            return Err(actix_web::error::ErrorBadRequest(
                "This assignment is not closed yet.",
            ));
        }
    } else {
        return Err(actix_web::error::ErrorNotFound("No such class."));
    }

    for score in req.into_inner() {
        sqlx::query("UPDATE `submissions` JOIN `users` ON `users`.`id` = `submissions`.`user_id` SET `score` = ? WHERE `users`.`code` = ? AND `class_id` = ?")
            .bind(&score.score)
            .bind(&score.user_code)
            .bind(class_id)
            .execute(&mut tx)
            .await
            .map_err(SqlxError)?;
    }

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::NoContent().finish())
}
