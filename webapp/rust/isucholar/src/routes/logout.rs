use actix_web::HttpResponse;

// POST /logout ログアウト
pub async fn logout(session: actix_session::Session) -> actix_web::Result<HttpResponse> {
    session.purge();
    Ok(HttpResponse::Ok().finish())
}
