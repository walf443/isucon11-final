use crate::responses::error::ResponseError::AnnouncementNotFound;
use crate::responses::error::ResponseResult;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::repos::registration_repository::{
    RegistrationRepository, RegistrationRepositoryImpl,
};
use isucholar_core::repos::unread_announcement_repository::{
    UnreadAnnouncementRepository, UnreadAnnouncementRepositoryImpl,
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
    let unread_announcement_repo = UnreadAnnouncementRepositoryImpl {};

    let announcement = unread_announcement_repo
        .find_announcement_detail_by_announcement_id_and_user_id_in_tx(
            &mut tx,
            announcement_id,
            &user_id,
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

    unread_announcement_repo
        .mark_read(&mut tx, announcement_id, &user_id)
        .await?;

    tx.commit().await?;

    Ok(HttpResponse::Ok().json(announcement))
}
