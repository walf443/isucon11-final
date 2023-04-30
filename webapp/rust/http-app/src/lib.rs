use crate::routes::course_routes::get_course_routes;
use crate::routes::initialize::initialize;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{web, Error};
use isucholar_core::db::DBPool;
use isucholar_http_core::middleware::IsLoggedIn;
use isucholar_http_core::routes::announcement_routes::get_announcement_routes;
use isucholar_http_core::routes::login::login;
use isucholar_http_core::routes::logout::logout;
use isucholar_http_core::routes::user_routes::get_user_routes;
use isucholar_infra::services::manager::ServiceManagerImpl;

pub mod routes;

const SESSION_NAME: &str = "isucholar_rust";

pub fn create_app(
    pool: DBPool,
    service: ServiceManagerImpl,
) -> actix_web::App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = Error,
        InitError = (),
    >,
> {
    let users_api = get_user_routes::<ServiceManagerImpl>();
    let courses_api = get_course_routes::<ServiceManagerImpl>();
    let announcements_api = get_announcement_routes::<ServiceManagerImpl>();

    let mut session_key = b"trapnomura".to_vec();
    session_key.resize(32, 0);

    actix_web::App::new()
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(service))
        // .wrap(actix_web::middleware::Logger::default())
        .wrap(
            actix_session::CookieSession::signed(&session_key)
                .secure(false)
                .name(SESSION_NAME)
                .max_age(3600),
        )
        .route("/initialize", web::post().to(initialize))
        .route("/login", web::post().to(login::<ServiceManagerImpl>))
        .route("/logout", web::post().to(logout))
        .service(
            web::scope("/api")
                .wrap(IsLoggedIn)
                .service(users_api)
                .service(courses_api)
                .service(announcements_api),
        )
}
