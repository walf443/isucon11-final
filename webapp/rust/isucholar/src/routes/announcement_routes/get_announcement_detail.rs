use crate::db;
use crate::responses::error::SqlxError;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::models::announcement_detail::AnnouncementDetail;

// GET /api/announcements/{announcement_id} お知らせ詳細取得
pub async fn get_announcement_detail(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    announcement_id: web::Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let announcement_id = &announcement_id.0;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let announcement: Option<AnnouncementDetail> = db::fetch_optional_as(
        sqlx::query_as(concat!(
        "SELECT `announcements`.`id`, `courses`.`id` AS `course_id`, `courses`.`name` AS `course_name`, `announcements`.`title`, `announcements`.`message`, NOT `unread_announcements`.`is_deleted` AS `unread`",
        " FROM `announcements`",
        " JOIN `courses` ON `courses`.`id` = `announcements`.`course_id`",
        " JOIN `unread_announcements` ON `unread_announcements`.`announcement_id` = `announcements`.`id`",
        " WHERE `announcements`.`id` = ?",
        " AND `unread_announcements`.`user_id` = ?",
        )).bind(announcement_id).bind(&user_id),
        &mut tx
    )
        .await
        .map_err(SqlxError)?;
    if announcement.is_none() {
        return Err(actix_web::error::ErrorNotFound("No such announcement."));
    }
    let announcement = announcement.unwrap();

    let registration_count: i64 = db::fetch_one_scalar(
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM `registrations` WHERE `course_id` = ? AND `user_id` = ?",
        )
        .bind(&announcement.course_id)
        .bind(&user_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if registration_count == 0 {
        return Err(actix_web::error::ErrorNotFound("No such announcement."));
    }

    sqlx::query("UPDATE `unread_announcements` SET `is_deleted` = true WHERE `announcement_id` = ? AND `user_id` = ?")
        .bind(announcement_id)
        .bind(&user_id)
        .execute(&mut tx)
        .await
        .map_err(SqlxError)?;

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::Ok().json(announcement))
}
