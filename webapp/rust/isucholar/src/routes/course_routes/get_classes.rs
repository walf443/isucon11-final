use crate::db;
use crate::responses::error::SqlxError;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::models::class::ClassWithSubmitted;

#[derive(Debug, serde::Serialize)]
struct GetClassResponse {
    id: String,
    part: u8,
    title: String,
    description: String,
    submission_closed: bool,
    submitted: bool,
}

// GET /api/courses/{course_id}/classes 科目に紐づく講義一覧の取得
pub async fn get_classes(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    course_id: web::Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_id = &course_id.0;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let count: i64 = db::fetch_one_scalar(
        sqlx::query_scalar("SELECT COUNT(*) FROM `courses` WHERE `id` = ?").bind(course_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if count == 0 {
        return Err(actix_web::error::ErrorNotFound("No such course."));
    }

    let classes: Vec<ClassWithSubmitted> = sqlx::query_as(concat!(
    "SELECT `classes`.*, `submissions`.`user_id` IS NOT NULL AS `submitted`",
    " FROM `classes`",
    " LEFT JOIN `submissions` ON `classes`.`id` = `submissions`.`class_id` AND `submissions`.`user_id` = ?",
    " WHERE `classes`.`course_id` = ?",
    " ORDER BY `classes`.`part`",
    ))
        .bind(&user_id)
        .bind(course_id)
        .fetch_all(&mut tx)
        .await
        .map_err(SqlxError)?;

    tx.commit().await.map_err(SqlxError)?;

    // 結果が0件の時は空配列を返却
    let res = classes
        .into_iter()
        .map(|class| GetClassResponse {
            id: class.id,
            part: class.part,
            title: class.title,
            description: class.description,
            submission_closed: class.submission_closed,
            submitted: class.submitted,
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(res))
}
