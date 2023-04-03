use crate::db;
use crate::responses::error::ResponseError::AnnouncementNotFound;
use crate::responses::error::ResponseResult;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::models::announcement_detail::AnnouncementDetail;
use isucholar_core::repos::registration_repository::{
    RegistrationRepository, RegistrationRepositoryImpl,
};

// GET /api/announcements/{announcement_id} お知らせ詳細取得
pub async fn get_announcement_detail(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    announcement_id: web::Path<(String,)>,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let announcement_id = &announcement_id.0;

    let mut tx = pool.begin().await?;

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
        .await?;
    if announcement.is_none() {
        return Err(AnnouncementNotFound);
    }
    let announcement = announcement.unwrap();

    let registration_repo = RegistrationRepositoryImpl {};
    let is_exist = registration_repo
        .exist_by_user_id_and_course_id_in_tx(&mut tx, &user_id, &announcement.course_id)
        .await?;
    if !is_exist {
        return Err(AnnouncementNotFound);
    }

    sqlx::query("UPDATE `unread_announcements` SET `is_deleted` = true WHERE `announcement_id` = ? AND `user_id` = ?")
        .bind(announcement_id)
        .bind(&user_id)
        .execute(&mut tx)
        .await?;

    tx.commit().await?;

    Ok(HttpResponse::Ok().json(announcement))
}
