use actix_web::web;
use sqlx::Executor;
use isucholar::routes::announcement_routes::add_announcement::add_announcement;
use isucholar::routes::announcement_routes::get_announcement_detail::get_announcement_detail;
use isucholar::routes::announcement_routes::get_announcement_list::get_announcement_list;
use isucholar::routes::course_routes::add_class::add_class;
use isucholar::routes::course_routes::add_course::add_course;
use isucholar::routes::course_routes::download_submitted_assignments::download_submitted_assignments;
use isucholar::routes::course_routes::get_classes::get_classes;
use isucholar::routes::course_routes::get_course_detail::get_course_detail;
use isucholar::routes::course_routes::register_scores::register_scores;
use isucholar::routes::course_routes::search_courses::search_courses;
use isucholar::routes::course_routes::set_course_status::set_course_status;
use isucholar::routes::course_routes::submit_assignment::submit_assignment;
use isucholar::routes::initialize::initialize;
use isucholar::routes::login::login;
use isucholar::routes::logout::logout;
use isucholar::routes::user_routes::get_grades::get_grades;
use isucholar::routes::user_routes::get_me::get_me;
use isucholar::routes::user_routes::get_registered_courses::get_registered_courses;
use isucholar::routes::user_routes::register_courses::register_courses;

const SESSION_NAME: &str = "isucholar_rust";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,sqlx=warn"))
        .init();

    let mysql_config = sqlx::mysql::MySqlConnectOptions::new()
        .host(
            &std::env::var("MYSQL_HOSTNAME")
                .ok()
                .unwrap_or_else(|| "127.0.0.1".to_owned()),
        )
        .port(
            std::env::var("MYSQL_PORT")
                .ok()
                .and_then(|port_str| port_str.parse().ok())
                .unwrap_or(3306),
        )
        .username(
            &std::env::var("MYSQL_USER")
                .ok()
                .unwrap_or_else(|| "isucon".to_owned()),
        )
        .password(
            &std::env::var("MYSQL_PASS")
                .ok()
                .unwrap_or_else(|| "isucon".to_owned()),
        )
        .database(
            &std::env::var("MYSQL_DATABASE")
                .ok()
                .unwrap_or_else(|| "isucholar".to_owned()),
        )
        .ssl_mode(sqlx::mysql::MySqlSslMode::Disabled);
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .after_connect(|conn| {
            Box::pin(async move {
                conn.execute("set time_zone = '+00:00'").await?;
                Ok(())
            })
        })
        .connect_with(mysql_config)
        .await
        .expect("failed to connect db");

    let mut session_key = b"trapnomura".to_vec();
    session_key.resize(32, 0);

    let server = actix_web::HttpServer::new(move || {
        let users_api = web::scope("/users")
            .route("/me", web::get().to(get_me))
            .route("/me/courses", web::get().to(get_registered_courses))
            .route("/me/courses", web::put().to(register_courses))
            .route("/me/grades", web::get().to(get_grades));

        let courses_api = web::scope("/courses")
            .route("", web::get().to(search_courses))
            .service(
                web::resource("")
                    .guard(actix_web::guard::Post())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(add_course),
            )
            .route("/{course_id}", web::get().to(get_course_detail))
            .service(
                web::resource("/{course_id}/status")
                    .guard(actix_web::guard::Put())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(set_course_status),
            )
            .route("/{course_id}/classes", web::get().to(get_classes))
            .service(
                web::resource("/{course_id}/classes")
                    .guard(actix_web::guard::Post())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(add_class),
            )
            .route(
                "/{course_id}/classes/{class_id}/assignments",
                web::post().to(submit_assignment),
            )
            .service(
                web::resource("/{course_id}/classes/{class_id}/assignments/scores")
                    .guard(actix_web::guard::Put())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(register_scores),
            )
            .service(
                web::resource("/{course_id}/classes/{class_id}/assignments/export")
                    .guard(actix_web::guard::Get())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(download_submitted_assignments),
            );

        let announcements_api = web::scope("/announcements")
            .route("", web::get().to(get_announcement_list))
            .service(
                web::resource("")
                    .guard(actix_web::guard::Post())
                    .wrap(isucholar::middleware::IsAdmin)
                    .to(add_announcement),
            )
            .route("/{announcement_id}", web::get().to(get_announcement_detail));

        actix_web::App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                actix_session::CookieSession::signed(&session_key)
                    .secure(false)
                    .name(SESSION_NAME)
                    .max_age(3600),
            )
            .route("/initialize", web::post().to(initialize))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .service(
                web::scope("/api")
                    .wrap(isucholar::middleware::IsLoggedIn)
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
