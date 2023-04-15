use crate::responses::error::ResponseError::AnnouncementNotFound;
use crate::responses::error::ResponseResult;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::services::error::Error;
use isucholar_core::services::unread_announcement_service::{
    HaveUnreadAnnouncementService, UnreadAnnouncementServiceVirtual,
};

// GET /api/announcements/{announcement_id} お知らせ詳細取得
pub async fn get_announcement_detail<Service: HaveUnreadAnnouncementService>(
    service: web::Data<Service>,
    session: actix_session::Session,
    announcement_id: web::Path<(String,)>,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let announcement_id: &str = &announcement_id.0;

    let result = service
        .unread_announcement_service()
        .find_detail_and_mark_read(announcement_id, &user_id)
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

#[cfg(test)]
mod tests {
    use crate::routes::announcement_routes::get_announcement_detail::get_announcement_detail;
    use actix_session::UserSession;
    use actix_web::test::TestRequest;
    use actix_web::web::{Data, Path};
    use actix_web::FromRequest;
    use isucholar_core::db::get_test_db_conn;
    use isucholar_core::services::manager::ServiceManagerImpl;

    #[actix_web::test]
    #[should_panic(expected = "AnnouncementNotFound")]
    async fn test_not_found_case() {
        let conn = get_test_db_conn().await.unwrap();
        let service = ServiceManagerImpl::new(conn.clone());

        let req = TestRequest::with_uri("/announcements/1")
            .param("announcement_id", "1".to_owned())
            .to_http_request();
        let announcement_id = Path::<(String,)>::extract(&req).await.unwrap();
        let session = req.get_session();
        let _ = session.insert("userID", "1");
        let _ = session.insert("userName", "1");
        let _ = session.insert("isAdmin", false);

        get_announcement_detail(Data::new(service), session, announcement_id)
            .await
            .unwrap();
    }
}
