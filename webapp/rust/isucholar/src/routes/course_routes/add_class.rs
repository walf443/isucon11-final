use crate::responses::error::ResponseError::{
    CourseConflict, CourseIsNotInProgress, CourseNotFound,
};
use crate::responses::error::ResponseResult;
use crate::util;
use actix_web::{web, HttpResponse};
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::repos::class_repository::{ClassRepository, ClassRepositoryImpl};
use isucholar_core::repos::course_repository::{CourseRepository, CourseRepositoryImpl};
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
) -> ResponseResult<HttpResponse> {
    let course_id = &course_id.0;

    let mut tx = pool.begin().await?;

    let course_repo = CourseRepositoryImpl {};
    let course = course_repo
        .find_for_share_lock_by_id_in_tx(&mut tx, course_id)
        .await?;
    if course.is_none() {
        return Err(CourseNotFound);
    }
    let course = course.unwrap();
    if course.status != CourseStatus::InProgress {
        return Err(CourseIsNotInProgress);
    }

    let class_repo = ClassRepositoryImpl {};
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
                    let class = class_repo
                        .find_by_course_id_and_part(&pool, course_id, &req.part)
                        .await?;
                    if req.title != class.title || req.description != class.description {
                        return Err(CourseConflict);
                    } else {
                        return Ok(
                            HttpResponse::Created().json(AddClassResponse { class_id: class.id })
                        );
                    }
                }
            }
        }
        return Err(e.into());
    }

    tx.commit().await?;

    Ok(HttpResponse::Created().json(AddClassResponse { class_id }))
}
