use crate::responses::error::ResponseError::{AnnouncementConflict, CourseNotFound};
use crate::responses::error::ResponseResult;
use actix_web::{web, HttpResponse};
use isucholar_core::models::announcement::Announcement;
use isucholar_core::models::user::User;
use isucholar_core::repos::course_repository::{CourseRepository, CourseRepositoryImpl};
use isucholar_core::MYSQL_ERR_NUM_DUPLICATE_ENTRY;

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

    let course_repo = CourseRepositoryImpl {};
    let is_exist = course_repo
        .exist_by_id_in_tx(&mut tx, &req.course_id)
        .await?;
    if !is_exist {
        return Err(CourseNotFound);
    }

    let result = sqlx::query(
        "INSERT INTO `announcements` (`id`, `course_id`, `title`, `message`) VALUES (?, ?, ?, ?)",
    )
    .bind(&req.id)
    .bind(&req.course_id)
    .bind(&req.title)
    .bind(&req.message)
    .execute(&mut tx)
    .await;
    if let Err(e) = result {
        let _ = tx.rollback().await;
        if let sqlx::error::Error::Database(ref db_error) = e {
            if let Some(mysql_error) =
                db_error.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>()
            {
                if mysql_error.number() == MYSQL_ERR_NUM_DUPLICATE_ENTRY {
                    let announcement: Announcement =
                        sqlx::query_as("SELECT * FROM `announcements` WHERE `id` = ?")
                            .bind(&req.id)
                            .fetch_one(pool.as_ref())
                            .await?;
                    if announcement.course_id != req.course_id
                        || announcement.title != req.title
                        || announcement.message != req.message
                    {
                        return Err(AnnouncementConflict);
                    } else {
                        return Ok(HttpResponse::Created().finish());
                    }
                }
            }
        }
        return Err(e.into());
    }

    let targets: Vec<User> = sqlx::query_as(concat!(
        "SELECT `users`.* FROM `users`",
        " JOIN `registrations` ON `users`.`id` = `registrations`.`user_id`",
        " WHERE `registrations`.`course_id` = ?",
    ))
    .bind(&req.course_id)
    .fetch_all(&mut tx)
    .await?;

    for user in targets {
        sqlx::query(
            "INSERT INTO `unread_announcements` (`announcement_id`, `user_id`) VALUES (?, ?)",
        )
        .bind(&req.id)
        .bind(user.id)
        .execute(&mut tx)
        .await?;
    }

    tx.commit().await?;

    Ok(HttpResponse::Created().finish())
}
