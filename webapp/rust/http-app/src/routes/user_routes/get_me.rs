use actix_web::{web, HttpResponse};
use isucholar_core::repos::user_repository::UserRepository;
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_http_core::routes::util::get_user_info;
use isucholar_infra::repos::user_repository::UserRepositoryImpl;

#[derive(Debug, serde::Serialize)]
pub struct GetMeResponse {
    code: String,
    name: String,
    is_admin: bool,
}

// GET /api/users/me 自身の情報を取得
pub async fn get_me(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
) -> ResponseResult<HttpResponse> {
    let (user_id, user_name, is_admin) = get_user_info(session)?;

    let user_repos = UserRepositoryImpl {};
    let user_code = user_repos.find_code_by_id(&pool, &user_id).await?;

    Ok(HttpResponse::Ok().json(GetMeResponse {
        code: user_code,
        name: user_name,
        is_admin,
    }))
}
