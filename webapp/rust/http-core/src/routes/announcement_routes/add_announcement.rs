use crate::responses::error::ResponseError::{AnnouncementConflict, CourseNotFound};
use crate::responses::error::ResponseResult;
use actix_web::{web, HttpResponse};
use isucholar_core::models::announcement::{Announcement, AnnouncementID};
use isucholar_core::models::course::CourseID;
use isucholar_core::services::announcement_service::{
    AnnouncementService, HaveAnnouncementService,
};
use isucholar_core::services::error::Error;

#[derive(Debug, serde::Deserialize)]
pub struct AddAnnouncementRequest {
    id: AnnouncementID,
    course_id: CourseID,
    title: String,
    message: String,
}

// POST /api/announcements 新規お知らせ追加
pub async fn add_announcement<Service: HaveAnnouncementService>(
    service: web::Data<Service>,
    req: web::Json<AddAnnouncementRequest>,
) -> ResponseResult<HttpResponse> {
    let announcement = Announcement {
        id: req.id.clone(),
        course_id: req.course_id.clone(),
        title: req.title.clone(),
        message: req.message.clone(),
    };

    let result = service.announcement_service().create(&announcement).await;
    return match result {
        Ok(_) => Ok(HttpResponse::Created().finish()),
        Err(e) => match e {
            Error::AnnouncementDuplicate => Err(AnnouncementConflict),
            Error::CourseNotFound => Err(CourseNotFound),
            _ => Err(e.into()),
        },
    };
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test::TestRequest;
    use actix_web::web::{Data, Json};
    use isucholar_core::models::announcement::AnnouncementID;
    use isucholar_core::models::course::CourseID;

    use crate::routes::announcement_routes::add_announcement::{
        add_announcement, AddAnnouncementRequest,
    };
    use isucholar_core::services::error::Error::{
        AnnouncementDuplicate, CourseNotFound, TestError,
    };
    use isucholar_core::services::manager::tests::MockServiceManager;

    #[actix_web::test]
    #[should_panic(expected = "CourseNotFound")]
    async fn test_course_not_found_case() {
        let mut service = MockServiceManager::new();
        service
            .announcement_service
            .expect_create()
            .returning(|_| Err(CourseNotFound));

        let _req = TestRequest::with_uri("/announcements").to_http_request();

        add_announcement(
            Data::new(service),
            Json(AddAnnouncementRequest {
                id: AnnouncementID::new("".to_string()),
                course_id: CourseID::new("".to_string()),
                title: "".to_string(),
                message: "".to_string(),
            }),
        )
        .await
        .unwrap();
    }

    #[actix_web::test]
    #[should_panic(expected = "AnnouncementConflict")]
    async fn test_conflict_case() {
        let mut service = MockServiceManager::new();
        service
            .announcement_service
            .expect_create()
            .returning(|_| Err(AnnouncementDuplicate));

        let _req = TestRequest::with_uri("/announcements").to_http_request();

        add_announcement(
            Data::new(service),
            Json(AddAnnouncementRequest {
                id: AnnouncementID::new("".to_string()),
                course_id: CourseID::new("".to_string()),
                title: "".to_string(),
                message: "".to_string(),
            }),
        )
        .await
        .unwrap();
    }

    #[actix_web::test]
    #[should_panic(expected = "ServiceError(TestError)")]
    async fn test_error() {
        let mut service = MockServiceManager::new();
        service
            .announcement_service
            .expect_create()
            .returning(|_| Err(TestError));

        let _req = TestRequest::with_uri("/announcements").to_http_request();

        add_announcement(
            Data::new(service),
            Json(AddAnnouncementRequest {
                id: AnnouncementID::new("".to_string()),
                course_id: CourseID::new("".to_string()),
                title: "".to_string(),
                message: "".to_string(),
            }),
        )
        .await
        .unwrap();
    }

    #[actix_web::test]
    async fn success_case() {
        let mut service = MockServiceManager::new();
        service
            .announcement_service
            .expect_create()
            .returning(|_| Ok(()));

        let _req = TestRequest::with_uri("/announcements").to_http_request();

        let result = add_announcement(
            Data::new(service),
            Json(AddAnnouncementRequest {
                id: AnnouncementID::new("".to_string()),
                course_id: CourseID::new("".to_string()),
                title: "".to_string(),
                message: "".to_string(),
            }),
        )
        .await
        .unwrap();

        assert_eq!(result.status(), StatusCode::CREATED);
    }
}
