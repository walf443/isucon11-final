use crate::responses::error::ResponseError::UserNotFound;
use crate::responses::error::ResponseResult;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::services::user_service::{HaveUserService, UserService};

#[derive(Debug, serde::Serialize)]
pub struct GetMeResponse {
    code: String,
    name: String,
    is_admin: bool,
}

// GET /api/users/me 自身の情報を取得
pub async fn get_me<Service: HaveUserService>(
    service: web::Data<Service>,
    session: actix_session::Session,
) -> ResponseResult<HttpResponse> {
    let (user_id, user_name, is_admin) = get_user_info(session)?;

    let user_code = service.user_service().find_code_by_id(&user_id).await?;

    match user_code {
        None => Err(UserNotFound),
        Some(user_code) => Ok(HttpResponse::Ok().json(GetMeResponse {
            code: user_code,
            name: user_name,
            is_admin,
        })),
    }
}

#[cfg(test)]
mod tests {
    use crate::routes::user_routes::get_me::{get_me, GetMeResponse};
    use actix_session::UserSession;
    use actix_web::body::to_bytes;
    use actix_web::http::StatusCode;
    use actix_web::test::TestRequest;
    use actix_web::web;
    use isucholar_core::services::error::Error::TestError;
    use isucholar_core::services::user_service::{HaveUserService, MockUserService};
    use std::str::from_utf8;

    struct S {
        user_service: MockUserService,
    }

    impl S {
        fn new() -> Self {
            Self {
                user_service: MockUserService::new(),
            }
        }
    }

    impl HaveUserService for S {
        type Service = MockUserService;

        fn user_service(&self) -> &Self::Service {
            &self.user_service
        }
    }

    #[actix_web::test]
    #[should_panic(expected = "UserNotFound")]
    async fn test_none_case() {
        let mut service = S::new();

        service
            .user_service
            .expect_find_code_by_id()
            .withf(|uid| uid == "1")
            .returning(|_| Ok(None));

        let req = TestRequest::with_uri("/user/me").to_http_request();
        let session = req.get_session();
        let _ = session.insert("userID", "1");
        let _ = session.insert("userName", "1");
        let _ = session.insert("isAdmin", false);

        get_me(web::Data::new(service), session).await.unwrap();
    }

    #[actix_web::test]
    #[should_panic(expected = "TestError")]
    async fn test_error_case() {
        let mut service = S::new();

        service
            .user_service
            .expect_find_code_by_id()
            .withf(|uid| uid == "1")
            .returning(|_| Err(TestError));

        let req = TestRequest::with_uri("/user/me").to_http_request();
        let session = req.get_session();
        let _ = session.insert("userID", "1");
        let _ = session.insert("userName", "1");
        let _ = session.insert("isAdmin", false);

        get_me(web::Data::new(service), session).await.unwrap();
    }

    #[actix_web::test]
    async fn test_success_case() {
        let mut service = S::new();

        service
            .user_service
            .expect_find_code_by_id()
            .withf(|uid| uid == "1")
            .returning(|_| Ok(Some("abc".to_string())));

        let req = TestRequest::with_uri("/user/me").to_http_request();
        let session = req.get_session();
        let _ = session.insert("userID", "1");
        let _ = session.insert("userName", "1");
        let _ = session.insert("isAdmin", false);

        let result = get_me(web::Data::new(service), session).await.unwrap();
        assert_eq!(result.status(), StatusCode::OK);
        let expected = GetMeResponse {
            code: "abc".to_string(),
            name: "1".to_string(),
            is_admin: false,
        };
        let expected = serde_json::to_string(&expected).unwrap();
        let body = to_bytes(result.into_body()).await.unwrap();

        assert_eq!(from_utf8(&body).unwrap(), expected)
    }
}
