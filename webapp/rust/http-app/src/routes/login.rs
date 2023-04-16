use actix_web::{web, HttpResponse};
use isucholar_core::models::user_type::UserType;
use isucholar_core::services::user_service::{HaveUserService, UserService};
use isucholar_http_core::responses::error::ResponseError::{AlreadyLogin, Unauthorized};
use isucholar_http_core::responses::error::ResponseResult;

#[derive(Debug, serde::Deserialize)]
pub struct LoginRequest {
    code: String,
    password: String,
}

// POST /login ログイン
pub async fn login<Service: HaveUserService>(
    service: web::Data<Service>,
    session: actix_session::Session,
    req: web::Json<LoginRequest>,
) -> ResponseResult<HttpResponse> {
    let user = service.user_service().find_by_code(&req.code).await?;

    if user.is_none() {
        return Err(Unauthorized);
    }
    let user = user.unwrap();

    service.user_service().verify_password(&user, &req.password)?;

    if let Some(user_id) = session.get::<String>("userID")? {
        if user_id == user.id {
            return Err(AlreadyLogin);
        }
    }

    session.insert("userID", user.id)?;
    session.insert("userName", user.name)?;
    session.insert("isAdmin", user.type_ == UserType::Teacher)?;
    Ok(HttpResponse::Ok().finish())
}
