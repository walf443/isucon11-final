use crate::responses::error::SqlxError;
use actix_web::{web, HttpResponse};
use isucholar_core::models::user::User;
use isucholar_core::models::user_type::UserType;

#[derive(Debug, serde::Deserialize)]
pub struct LoginRequest {
    code: String,
    password: String,
}

// POST /login ログイン
pub async fn login(
    session: actix_session::Session,
    pool: web::Data<sqlx::MySqlPool>,
    req: web::Json<LoginRequest>,
) -> actix_web::Result<HttpResponse> {
    let user: Option<User> = sqlx::query_as("SELECT * FROM `users` WHERE `code` = ?")
        .bind(&req.code)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(SqlxError)?;
    if user.is_none() {
        return Err(actix_web::error::ErrorUnauthorized(
            "Code or Password is wrong.",
        ));
    }
    let user = user.unwrap();

    if !bcrypt::verify(
        &req.password,
        &String::from_utf8(user.hashed_password).unwrap(),
    )
    .map_err(actix_web::error::ErrorInternalServerError)?
    {
        return Err(actix_web::error::ErrorUnauthorized(
            "Code or Password is wrong.",
        ));
    }

    if let Some(user_id) = session.get::<String>("userID")? {
        if user_id == user.id {
            return Err(actix_web::error::ErrorBadRequest(
                "You are already logged in.",
            ));
        }
    }

    session.insert("userID", user.id)?;
    session.insert("userName", user.name)?;
    session.insert("isAdmin", user.type_ == UserType::Teacher)?;
    Ok(HttpResponse::Ok().finish())
}
