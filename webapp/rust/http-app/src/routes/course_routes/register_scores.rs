use crate::responses::error::ResponseError::{ClassIsNotSubmissionClosed, ClassNotFound};
use crate::responses::error::ResponseResult;
use actix_web::{web, HttpResponse};
use isucholar_core::models::assignment_path::AssignmentPath;
use isucholar_core::models::score::Score;
use isucholar_core::repos::class_repository::{ClassRepository};
use isucholar_core::repos::submission_repository::{
    SubmissionRepository, SubmissionRepositoryImpl,
};
use isucholar_infra::repos::class_repository::ClassRepositoryImpl;

// PUT /api/courses/{course_id}/classes/{class_id}/assignments/scores 採点結果登録
pub async fn register_scores(
    pool: web::Data<sqlx::MySqlPool>,
    path: web::Path<AssignmentPath>,
    req: web::Json<Vec<Score>>,
) -> ResponseResult<HttpResponse> {
    let class_id = &path.class_id;

    let mut tx = pool.begin().await?;
    let class_repo = ClassRepositoryImpl {};
    let submission_closed = class_repo
        .find_submission_closed_by_id_in_tx(&mut tx, class_id)
        .await?;

    if let Some(submission_closed) = submission_closed {
        if !submission_closed {
            return Err(ClassIsNotSubmissionClosed);
        }
    } else {
        return Err(ClassNotFound);
    }

    let submission_repo = SubmissionRepositoryImpl {};

    for score in req.into_inner() {
        submission_repo
            .update_score_by_user_code_and_class_id(
                &mut tx,
                &score.user_code,
                &class_id,
                score.score,
            )
            .await?;
    }

    tx.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}
