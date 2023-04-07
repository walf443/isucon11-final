use crate::responses::error::ResponseError::AnnouncementNotFound;
use crate::responses::error::ResponseResult;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::services::error::Error;
use isucholar_core::services::manager::ServiceManager;
use isucholar_core::services::unread_announcement_service::UnreadAnnouncementService;

// GET /api/announcements/{announcement_id} お知らせ詳細取得
pub async fn get_announcement_detail<Service: ServiceManager>(
    pool: web::Data<sqlx::MySqlPool>,
    service: web::Data<Service>,
    session: actix_session::Session,
    announcement_id: web::Path<(String,)>,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let announcement_id = &announcement_id.0;

    let result = service
        .unread_announcement_service()
        .find_detail_and_mark_read(&pool, announcement_id, &user_id)
        .await;
    match result {
        Ok(_) => {}
        Err(e) => {
            return match e {
                Error::AnnouncementNotFound => Err(AnnouncementNotFound),
                _ => Err(e.into()),
            }
        }
    }
    let announcement = result.unwrap();

    Ok(HttpResponse::Ok().json(announcement))
}
