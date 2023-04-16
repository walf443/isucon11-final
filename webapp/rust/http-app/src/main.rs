use actix_web::web;

use isucholar_http_app::routes::course_routes::get_course_routes;
use isucholar_http_app::routes::initialize::initialize;
use isucholar_http_app::routes::user_routes::get_user_routes;
use isucholar_http_core::middleware::IsLoggedIn;
use isucholar_http_core::routes::announcement_routes::get_announcement_routes;
use isucholar_http_core::routes::login::login;
use isucholar_http_core::routes::logout::logout;
use isucholar_infra::db::get_db_conn;
use isucholar_infra::services::manager::ServiceManagerImpl;

const SESSION_NAME: &str = "isucholar_rust";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,sqlx=warn"))
        .init();

    let pool = get_db_conn().await.expect("failed to connect db");
    let service = ServiceManagerImpl::new(pool.clone());

    let mut session_key = b"trapnomura".to_vec();
    session_key.resize(32, 0);

    let server = actix_web::HttpServer::new(move || {
        let users_api = get_user_routes();
        let courses_api = get_course_routes();
        let announcements_api = get_announcement_routes::<ServiceManagerImpl>();

        actix_web::App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(service.clone()))
            .wrap(actix_web::middleware::Logger::default())
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
    });
    if let Some(l) = listenfd::ListenFd::from_env().take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind((
            "0.0.0.0",
            std::env::var("PORT")
                .ok()
                .and_then(|port_str| port_str.parse().ok())
                .unwrap_or(7000),
        ))?
    }
    .run()
    .await
}
