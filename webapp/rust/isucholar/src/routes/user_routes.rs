use crate::responses::error::SqlxError;
use crate::responses::get_me_response::GetMeResponse;
use actix_web::{web, HttpResponse};

fn get_user_info(session: actix_session::Session) -> actix_web::Result<(String, String, bool)> {
    let user_id = session.get("userID")?;
    if user_id.is_none() {
        return Err(actix_web::error::ErrorInternalServerError(
            "failed to get userID from session",
        ));
    }
    let user_name = session.get("userName")?;
    if user_name.is_none() {
        return Err(actix_web::error::ErrorInternalServerError(
            "failed to get userName from session",
        ));
    }
    let is_admin = session.get("isAdmin")?;
    if is_admin.is_none() {
        return Err(actix_web::error::ErrorInternalServerError(
            "failed to get isAdmin from session",
        ));
    }
    Ok((user_id.unwrap(), user_name.unwrap(), is_admin.unwrap()))
}

// GET /api/users/me 自身の情報を取得
pub async fn get_me(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
) -> actix_web::Result<HttpResponse> {
    let (user_id, user_name, is_admin) = get_user_info(session)?;

    let user_code = sqlx::query_scalar("SELECT `code` FROM `users` WHERE `id` = ?")
        .bind(&user_id)
        .fetch_one(pool.as_ref())
        .await
        .map_err(SqlxError)?;

    Ok(HttpResponse::Ok().json(GetMeResponse {
        code: user_code,
        name: user_name,
        is_admin,
    }))
}
