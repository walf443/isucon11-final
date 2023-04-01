use crate::responses::error::SqlxError;
use crate::{db, util};
use actix_web::{web, HttpResponse};
use isucholar_core::models::class::Class;
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::MYSQL_ERR_NUM_DUPLICATE_ENTRY;

#[derive(Debug, serde::Deserialize)]
pub struct AddClassRequest {
    part: u8,
    title: String,
    description: String,
}

#[derive(Debug, serde::Serialize)]
struct AddClassResponse {
    class_id: String,
}

// POST /api/courses/{course_id}/classes 新規講義(&課題)追加
pub async fn add_class(
    pool: web::Data<sqlx::MySqlPool>,
    course_id: web::Path<(String,)>,
    req: web::Json<AddClassRequest>,
) -> actix_web::Result<HttpResponse> {
    let course_id = &course_id.0;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let course: Option<Course> = db::fetch_optional_as(
        sqlx::query_as("SELECT * FROM `courses` WHERE `id` = ? FOR SHARE").bind(course_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if course.is_none() {
        return Err(actix_web::error::ErrorNotFound("No such course."));
    }
    let course = course.unwrap();
    if course.status != CourseStatus::InProgress {
        return Err(actix_web::error::ErrorBadRequest(
            "This course is not in-progress.",
        ));
    }

    let class_id = util::new_ulid().await;
    let result = sqlx::query("INSERT INTO `classes` (`id`, `course_id`, `part`, `title`, `description`) VALUES (?, ?, ?, ?, ?)")
        .bind(&class_id)
        .bind(course_id)
        .bind(&req.part)
        .bind(&req.title)
        .bind(&req.description)
        .execute(&mut tx)
        .await;
    if let Err(e) = result {
        let _ = tx.rollback().await;
        if let sqlx::error::Error::Database(ref db_error) = e {
            if let Some(mysql_error) =
                db_error.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>()
            {
                if mysql_error.number() == MYSQL_ERR_NUM_DUPLICATE_ENTRY {
                    let class: Class = sqlx::query_as(
                        "SELECT * FROM `classes` WHERE `course_id` = ? AND `part` = ?",
                    )
                    .bind(course_id)
                    .bind(&req.part)
                    .fetch_one(pool.as_ref())
                    .await
                    .map_err(SqlxError)?;
                    if req.title != class.title || req.description != class.description {
                        return Err(actix_web::error::ErrorConflict(
                            "A class with the same part already exists.",
                        ));
                    } else {
                        return Ok(
                            HttpResponse::Created().json(AddClassResponse { class_id: class.id })
                        );
                    }
                }
            }
        }
        return Err(SqlxError(e).into());
    }

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::Created().json(AddClassResponse { class_id }))
}
