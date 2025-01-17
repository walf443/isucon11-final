extern crate isucholar_http_app;

use actix_web::http::StatusCode;
use actix_web::test;
use isucholar_core::db::get_test_db_conn;
use isucholar_http_app::create_app;
use isucholar_infra::services::manager::ServiceManagerInfra;

#[actix_web::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let service = ServiceManagerInfra::new(db_pool.clone());

    let app = create_app(db_pool, service);

    let app = test::init_service(app).await;
    let req = test::TestRequest::post().uri("/logout").to_request();

    let res = test::call_service(&app, req).await;
    assert_eq!(res.status(), StatusCode::OK);
}
