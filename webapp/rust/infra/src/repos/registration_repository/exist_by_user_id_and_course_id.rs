use crate::repos::registration_repository::RegistrationRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::course::CourseID;
use isucholar_core::models::user::UserID;
use isucholar_core::repos::registration_repository::RegistrationRepository;
use sqlx::Acquire;

#[tokio::test]
async fn false_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let user_id: UserID = Faker.fake();
    let course_id: CourseID = Faker.fake();

    let repo = RegistrationRepositoryInfra {};
    let got = repo
        .exist_by_user_id_and_course_id(conn, &user_id, &course_id)
        .await
        .unwrap();
    assert_eq!(got, false);
}

#[tokio::test]
async fn true_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let user_id: UserID = Faker.fake();
    let course_id: CourseID = Faker.fake();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(conn)
        .await
        .unwrap();
    let conn = tx.acquire().await.unwrap();
    sqlx::query!(
        "INSERT INTO registrations (course_id, user_id) VALUES (?, ?)",
        &course_id,
        &user_id,
    )
    .execute(conn)
    .await
    .unwrap();

    let repo = RegistrationRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    let got = repo
        .exist_by_user_id_and_course_id(conn, &user_id, &course_id)
        .await
        .unwrap();
    assert_eq!(got, true);
}
