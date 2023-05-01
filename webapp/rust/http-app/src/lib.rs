use crate::routes::course_routes::get_course_routes;
use crate::routes::initialize::initialize;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::body::{BoxBody, MessageBody};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
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

pub fn create_app(
    pool: DBPool,
    service: ServiceManagerImpl,
) -> actix_web::App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<BoxBody>,
        Error = Error,
        InitError = (),
    >,
> {
    let users_api = get_user_routes::<ServiceManagerImpl>();
    let courses_api = get_course_routes::<ServiceManagerImpl>();
    let announcements_api = get_announcement_routes::<ServiceManagerImpl>();

    let session_key = env!("SESSION_KEY").try_into_bytes().unwrap().to_vec();

    actix_web::App::new()
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(service))
        // .wrap(actix_web::middleware::Logger::default())
        .wrap(
            SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&session_key))
                .cookie_secure(false)
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(1)))
                .build(),
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
