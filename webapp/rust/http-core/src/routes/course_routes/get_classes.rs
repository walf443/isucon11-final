use crate::responses::error::ResponseResult;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::models::class::ClassID;
use isucholar_core::models::course::CourseID;
use isucholar_core::services::class_service::{ClassService, HaveClassService};

#[derive(Debug, serde::Serialize)]
struct GetClassResponse {
    id: ClassID,
    part: u8,
    title: String,
    description: String,
    submission_closed: bool,
    submitted: bool,
}

// GET /api/courses/{course_id}/classes 科目に紐づく講義一覧の取得
pub async fn get_classes<Service: HaveClassService>(
    service: web::Data<Service>,
    session: actix_session::Session,
    course_id: web::Path<(String,)>,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_id = &course_id.0;
    let course_id = CourseID::new(course_id.to_string());

    let classes = service
        .class_service()
        .find_all_with_submitted_by_user_id_and_course_id(&user_id, &course_id)
        .await?;

    // 結果が0件の時は空配列を返却
    let res = classes
        .into_iter()
        .map(|class| GetClassResponse {
            id: class.id,
            part: class.part,
            title: class.title,
            description: class.description,
            submission_closed: class.submission_closed,
            submitted: class.submitted,
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(res))
}
