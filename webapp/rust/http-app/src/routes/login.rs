use crate::responses::error::ResponseError::{AlreadyLogin, Unauthorized};
use crate::responses::error::ResponseResult;
use actix_web::{web, HttpResponse};
use isucholar_core::models::user_type::UserType;
use isucholar_core::repos::user_repository::{UserRepository, UserRepositoryImpl};

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
) -> ResponseResult<HttpResponse> {
    let user_repo = UserRepositoryImpl {};
    let user = user_repo.find_by_code(&pool, &req.code).await?;
    if user.is_none() {
        return Err(Unauthorized);
    }
    let user = user.unwrap();

    if !bcrypt::verify(
        &req.password,
        &String::from_utf8(user.hashed_password).unwrap(),
    )? {
        return Err(Unauthorized);
    }

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
