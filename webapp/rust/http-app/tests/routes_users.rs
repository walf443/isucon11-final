use actix_web::test;
use isucholar_core::db::get_test_db_conn;
use isucholar_http_app::create_app;
use isucholar_infra::services::manager::ServiceManagerInfra;

#[actix_web::test]
#[should_panic(expected = "You are not logged in.")]
async fn get_me() {
    let db_pool = get_test_db_conn().await.unwrap();
    let service = ServiceManagerInfra::new(db_pool.clone());

    let app = create_app(db_pool, service);

    let app = test::init_service(app).await;
    let req = test::TestRequest::get().uri("/api/users/me").to_request();

    test::call_service(&app, req).await;
}

#[actix_web::test]
#[should_panic(expected = "You are not logged in.")]
async fn get_me_courses() {
    let db_pool = get_test_db_conn().await.unwrap();
    let service = ServiceManagerInfra::new(db_pool.clone());

    let app = create_app(db_pool, service);

    let app = test::init_service(app).await;
    let req = test::TestRequest::get()
        .uri("/api/users/me/courses")
        .to_request();

    test::call_service(&app, req).await;
}

#[actix_web::test]
#[should_panic(expected = "You are not logged in.")]
async fn put_me_courses() {
    let db_pool = get_test_db_conn().await.unwrap();
    let service = ServiceManagerInfra::new(db_pool.clone());

    let app = create_app(db_pool, service);

    let app = test::init_service(app).await;
    let req = test::TestRequest::put()
        .uri("/api/users/me/courses")
        .to_request();

    test::call_service(&app, req).await;
}

#[actix_web::test]
#[should_panic(expected = "You are not logged in.")]
async fn get_me_grades() {
    let db_pool = get_test_db_conn().await.unwrap();
    let service = ServiceManagerInfra::new(db_pool.clone());

    let app = create_app(db_pool, service);

    let app = test::init_service(app).await;
    let req = test::TestRequest::get()
        .uri("/api/users/me/grades")
        .to_request();

    test::call_service(&app, req).await;
}
