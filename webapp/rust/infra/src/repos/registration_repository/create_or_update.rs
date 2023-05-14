use crate::repos::registration_repository::RegistrationRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::course::CourseID;
use isucholar_core::models::user::UserID;
use isucholar_core::repos::registration_repository::RegistrationRepository;
use sqlx::Acquire;

#[tokio::test]
async fn insert_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(conn)
        .await
        .unwrap();

    let user_id: UserID = Faker.fake();
    let course_id: CourseID = Faker.fake();
    let repo = RegistrationRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    repo.create_or_update(conn, &user_id, &course_id)
        .await
        .unwrap();

    let conn = tx.acquire().await.unwrap();
    let row_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM registrations WHERE user_id = ? AND course_id = ?",
        &user_id,
        &course_id
    )
    .fetch_one(conn)
    .await
    .unwrap();

    assert_eq!(row_count, 1);
}

#[tokio::test]
async fn update_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(conn)
        .await
        .unwrap();

    let user_id: UserID = Faker.fake();
    let course_id: CourseID = Faker.fake();
    let repo = RegistrationRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    repo.create_or_update(conn, &user_id, &course_id)
        .await
        .unwrap();

    let conn = tx.acquire().await.unwrap();
    repo.create_or_update(conn, &user_id, &course_id)
        .await
        .unwrap();

    let conn = tx.acquire().await.unwrap();
    let row_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM registrations WHERE user_id = ? AND course_id = ?",
        &user_id,
        &course_id
    )
    .fetch_one(conn)
    .await
    .unwrap();

    assert_eq!(row_count, 1);
}
