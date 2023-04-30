use isucholar_http_app::create_app;

use isucholar_infra::db::get_db_conn;
use isucholar_infra::services::manager::ServiceManagerImpl;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info,sqlx=warn"))
        .init();

    let pool = get_db_conn().await.expect("failed to connect db");
    let service = ServiceManagerImpl::new(pool.clone());

    let server = actix_web::HttpServer::new(move || {
        let app = create_app(pool.clone(), service.clone());
        app.wrap(actix_web::middleware::Logger::default())
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
