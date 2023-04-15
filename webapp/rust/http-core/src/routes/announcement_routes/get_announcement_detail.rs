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
    use actix_web::body::to_bytes;
    use actix_web::test::TestRequest;
    use actix_web::web::{Bytes, Data, Path};
    use actix_web::FromRequest;
    use isucholar_core::models::announcement_detail::AnnouncementDetail;
    use isucholar_core::services::error::Error::{AnnouncementNotFound, TestError};
    use isucholar_core::services::unread_announcement_service::{
        MockHaveUnreadAnnouncementService, MockUnreadAnnouncementServiceVirtual,
    };
    use std::str::from_utf8;

    fn wrap_manager(
        service: MockUnreadAnnouncementServiceVirtual,
    ) -> MockHaveUnreadAnnouncementService {
        let mut manager = MockHaveUnreadAnnouncementService::new();
        manager
            .expect_unread_announcement_service()
            .return_const(service);

        manager
    }

    #[actix_web::test]
    #[should_panic(expected = "AnnouncementNotFound")]
    async fn test_not_found_case() {
        let mut service = MockUnreadAnnouncementServiceVirtual::new();
        service
            .expect_find_detail_and_mark_read()
            .returning(|_, _| Err(AnnouncementNotFound));

        let manager = wrap_manager(service);

        let req = TestRequest::with_uri("/announcements/1")
            .param("announcement_id", "1".to_owned())
            .to_http_request();
        let announcement_id = Path::<(String,)>::extract(&req).await.unwrap();
        let session = req.get_session();
        let _ = session.insert("userID", "1");
        let _ = session.insert("userName", "1");
        let _ = session.insert("isAdmin", false);

        get_announcement_detail(Data::new(manager), session, announcement_id)
            .await
            .unwrap();
    }

    #[actix_web::test]
    #[should_panic(expected = "ServiceError(TestError)")]
    async fn test_err_but_not_not_found_error() {
        let mut service = MockUnreadAnnouncementServiceVirtual::new();
        service
            .expect_find_detail_and_mark_read()
            .returning(|_, _| Err(TestError));

        let manager = wrap_manager(service);

        let req = TestRequest::with_uri("/announcements/1")
            .param("announcement_id", "1".to_owned())
            .to_http_request();
        let announcement_id = Path::<(String,)>::extract(&req).await.unwrap();
        let session = req.get_session();
        let _ = session.insert("userID", "1");
        let _ = session.insert("userName", "1");
        let _ = session.insert("isAdmin", false);

        get_announcement_detail(Data::new(manager), session, announcement_id)
            .await
            .unwrap();
    }

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            from_utf8(self).unwrap()
        }
    }

    #[actix_web::test]
    async fn success() {
        let mut service = MockUnreadAnnouncementServiceVirtual::new();
        service
            .expect_find_detail_and_mark_read()
            .returning(|_, _| {
                Ok(AnnouncementDetail {
                    id: "id".to_string(),
                    course_id: "course_id".to_string(),
                    course_name: "course_name".to_string(),
                    title: "title".to_string(),
                    message: "message".to_string(),
                    unread: false,
                })
            });

        let manager = wrap_manager(service);

        let req = TestRequest::with_uri("/announcements/1")
            .param("announcement_id", "1".to_owned())
            .to_http_request();
        let announcement_id = Path::<(String,)>::extract(&req).await.unwrap();
        let session = req.get_session();
        let _ = session.insert("userID", "1");
        let _ = session.insert("userName", "1");
        let _ = session.insert("isAdmin", false);

        let result = get_announcement_detail(Data::new(manager), session, announcement_id)
            .await
            .unwrap();

        assert_eq!(result.status(), 200);

        eprintln!("{:?}", result);
        eprintln!("{:?}", result.body());
        let expected = AnnouncementDetail {
            id: "id".to_string(),
            course_id: "course_id".to_string(),
            course_name: "course_name".to_string(),
            title: "title".to_string(),
            message: "message".to_string(),
            unread: false,
        };
        let expected = serde_json::to_string(&expected).unwrap();

        let body = to_bytes(result.into_body()).await.unwrap();

        assert_eq!(body.as_str(), expected);
    }
}
