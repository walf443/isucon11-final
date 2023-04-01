use actix_web::HttpResponse;
use isucholar_core::repos::error::ReposError;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("repos error")]
    ReposError(#[from] ReposError),
    #[error("actix error")]
    ActixError(#[from] actix_web::Error)
}

impl actix_web::ResponseError for ResponseError {
    fn error_response(&self) -> HttpResponse {
        log::error!("{}", self);
        HttpResponse::InternalServerError()
            .content_type(mime::TEXT_PLAIN)
            .body(self.to_string())
    }
}

pub type ResponseResult<T> = std::result::Result<T, ResponseError>;

#[derive(Debug)]
pub struct SqlxError(pub sqlx::Error);
impl std::fmt::Display for SqlxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl actix_web::ResponseError for SqlxError {
    fn error_response(&self) -> HttpResponse {
        log::error!("{}", self);
        HttpResponse::InternalServerError()
            .content_type(mime::TEXT_PLAIN)
            .body(format!("SQLx error: {:?}", self.0))
    }
}
