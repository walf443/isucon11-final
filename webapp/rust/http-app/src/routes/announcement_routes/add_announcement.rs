use actix_web::{web, HttpResponse};
use isucholar_core::models::announcement::Announcement;
use isucholar_core::services::announcement_service::{
    AnnouncementService, HaveAnnouncementService,
};
use isucholar_core::services::error::Error;
use isucholar_http_core::responses::error::ResponseError::{AnnouncementConflict, CourseNotFound};
use isucholar_http_core::responses::error::ResponseResult;

#[derive(Debug, serde::Deserialize)]
pub struct AddAnnouncementRequest {
    id: String,
    course_id: String,
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
