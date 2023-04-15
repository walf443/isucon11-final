use crate::responses::error::ResponseError::{AnnouncementConflict, CourseNotFound};
use crate::responses::error::ResponseResult;
use actix_web::{web, HttpResponse};
use isucholar_core::models::announcement::Announcement;
use isucholar_core::repos::announcement_repository::{
    AnnouncementRepository, AnnouncementRepositoryImpl,
};
use isucholar_core::repos::course_repository::{CourseRepository, CourseRepositoryImpl};
use isucholar_core::repos::error::ReposError;
use isucholar_core::repos::registration_repository::{
    RegistrationRepository, RegistrationRepositoryImpl,
};
use isucholar_core::repos::unread_announcement_repository::{
    UnreadAnnouncementRepository, UnreadAnnouncementRepositoryImpl,
};

#[derive(Debug, serde::Deserialize)]
pub struct AddAnnouncementRequest {
    id: String,
    course_id: String,
    title: String,
    message: String,
}

// POST /api/announcements 新規お知らせ追加
pub async fn add_announcement(
    pool: web::Data<sqlx::MySqlPool>,
    req: web::Json<AddAnnouncementRequest>,
) -> ResponseResult<HttpResponse> {
    let mut tx = pool.begin().await?;

    let announcement_repos = AnnouncementRepositoryImpl {};
    let course_repo = CourseRepositoryImpl {};
    let is_exist = course_repo
        .exist_by_id_in_tx(&mut tx, &req.course_id)
        .await?;
    if !is_exist {
        return Err(CourseNotFound);
    }

    let result = announcement_repos
        .create_in_tx(
            &mut tx,
            &Announcement {
                id: req.id.clone(),
                course_id: req.course_id.clone(),
                title: req.title.clone(),
                message: req.message.clone(),
            },
        )
        .await;

    match result {
        Ok(_) => {}
        Err(e) => {
            let _ = tx.rollback().await;
            match e {
                ReposError::AnnouncementDuplicate => {
                    let announcement = announcement_repos.find_by_id(&pool, &req.id).await?;
                    if announcement.course_id != req.course_id
                        || announcement.title != req.title
                        || announcement.message != req.message
                    {
                        return Err(AnnouncementConflict);
                    } else {
                        return Ok(HttpResponse::Created().finish());
                    }
                }
                _ => return Err(e.into()),
            }
        }
    }

    let registration_repo = RegistrationRepositoryImpl {};
    let targets = registration_repo
        .find_users_by_course_id_in_tx(&mut tx, &req.course_id)
        .await?;

    let unread_announcement_repo = UnreadAnnouncementRepositoryImpl {};
    for user in targets {
        unread_announcement_repo
            .create_in_tx(&mut tx, &req.id, &user.id)
            .await?;
    }

    tx.commit().await?;

    Ok(HttpResponse::Created().finish())
}
