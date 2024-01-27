use crate::responses::error::ResponseResult;
use crate::responses::get_registered_course_response::GetRegisteredCourseResponseContent;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::services::course_service::{CourseService, HaveCourseService};

// GET /api/users/me/courses 履修中の科目一覧取得
pub async fn get_registered_courses<Service: HaveCourseService>(
    service: web::Data<Service>,
    session: actix_session::Session,
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_with_teachers = service
        .course_service()
        .find_open_courses_by_user_id(&user_id)
        .await?;

    // 履修科目が0件の時は空配列を返却
    let mut res = Vec::with_capacity(course_with_teachers.len());
    for (course, teacher) in course_with_teachers {
        res.push(GetRegisteredCourseResponseContent {
            id: course.id,
            name: course.name,
            teacher: teacher.name,
            period: course.period,
            day_of_week: course.day_of_week,
        });
    }

    Ok(HttpResponse::Ok().json(res))
}

#[cfg(test)]
mod tests {
    use crate::routes::user_routes::get_registered_courses::get_registered_courses;
    use actix_session::SessionExt;
    use actix_web::body::to_bytes;
    use actix_web::http::StatusCode;
    use actix_web::test::TestRequest;
    use actix_web::web;
    use isucholar_core::services::error::Error::TestError;
    use isucholar_core::services::manager::tests::MockServiceManager;
    use std::str::from_utf8;

    #[actix_web::test]
    #[should_panic(expected = "TestError")]
    async fn test_error_case() {
        let mut service = MockServiceManager::new();

        service
            .course_service
            .expect_find_open_courses_by_user_id()
            .withf(|uid| uid.inner() == "1")
            .returning(|_| Err(TestError));

        let req = TestRequest::with_uri("/users/me/courses").to_http_request();
        let session = req.get_session();
        let _ = session.insert("userID", "1");
        let _ = session.insert("userName", "1");
        let _ = session.insert("isAdmin", false);

        get_registered_courses(web::Data::new(service), session)
            .await
            .unwrap();
    }

    #[actix_web::test]
    async fn test_empty_case() {
        let mut service = MockServiceManager::new();

        service
            .course_service
            .expect_find_open_courses_by_user_id()
            .withf(|uid| uid.inner() == "1")
            .returning(|_| Ok(Vec::new()));

        let req = TestRequest::with_uri("/users/me/courses").to_http_request();
        let session = req.get_session();
        let _ = session.insert("userID", "1");
        let _ = session.insert("userName", "1");
        let _ = session.insert("isAdmin", false);

        let result = get_registered_courses(web::Data::new(service), session)
            .await
            .unwrap();
        assert_eq!(result.status(), StatusCode::OK);
        let body = to_bytes(result.into_body()).await.unwrap();
        assert_eq!(from_utf8(&body).unwrap(), "[]");
    }
}
