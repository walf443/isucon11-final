use actix_web::{web, HttpResponse};
use isucholar_core::models::class::CreateClass;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::repos::class_repository::ClassRepository;
use isucholar_core::repos::course_repository::CourseRepository;
use isucholar_core::repos::error::ReposError;
use isucholar_core::util;
use isucholar_http_core::responses::error::ResponseError::{
    CourseConflict, CourseIsNotInProgress, CourseNotFound,
};
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_infra::repos::class_repository::ClassRepositoryInfra;
use isucholar_infra::repos::course_repository::CourseRepositoryInfra;

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

    let course_repo = CourseRepositoryInfra {};
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

    let class_repo = ClassRepositoryInfra {};
    let class_id = util::new_ulid().await;
    let form = CreateClass {
        id: class_id.clone(),
        course_id: course_id.clone(),
        part: req.part.clone(),
        title: req.title.clone(),
        description: req.description.clone(),
    };
    let result = class_repo.create_in_tx(&mut tx, &form).await;
    match result {
        Ok(_) => {
            tx.commit().await?;
        }
        Err(e) => {
            let _ = tx.rollback().await;
            match e {
                ReposError::CourseDuplicate => {
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
                _ => {}
            }
        }
    }

    Ok(HttpResponse::Created().json(AddClassResponse { class_id }))
}
