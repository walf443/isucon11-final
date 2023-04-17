use actix_web::{web, HttpResponse};
use isucholar_core::services::user_service::{HaveUserService, UserService};
use isucholar_http_core::responses::error::ResponseError::UserNotFound;
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_http_core::routes::util::get_user_info;

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
