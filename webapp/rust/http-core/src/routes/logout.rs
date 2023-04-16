use actix_web::HttpResponse;

// POST /logout ログアウト
pub async fn logout(session: actix_session::Session) -> actix_web::Result<HttpResponse> {
    session.purge();
    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use crate::routes::logout::logout;
    use actix_session::UserSession;
    use actix_web::http::StatusCode;
    use actix_web::test::TestRequest;

    #[actix_web::test]
    async fn success_case() {
        let req = TestRequest::with_uri("/logout").to_http_request();

        let session = req.get_session();
        let _ = session.insert("userID", "1");
        let _ = session.insert("userName", "1");
        let _ = session.insert("isAdmin", false);

        let res = logout(session).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
}
