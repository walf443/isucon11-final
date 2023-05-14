use crate::repos::registration_repository::RegistrationRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::course::CourseID;
use isucholar_core::models::user::User;
use isucholar_core::repos::registration_repository::RegistrationRepository;
use sqlx::Acquire;

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let course_id: CourseID = Faker.fake();

    let repo = RegistrationRepositoryInfra {};
    let got = repo
        .find_users_by_course_id(conn, &course_id)
        .await
        .unwrap();
    assert_eq!(got.len(), 0);
}

#[tokio::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let course_id: CourseID = Faker.fake();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(conn)
        .await
        .unwrap();
    let mut user: User = Faker.fake();
    user.hashed_password.resize(60, 0);

    let conn = tx.acquire().await.unwrap();
    sqlx::query!(
        "INSERT INTO users (id, code, name, hashed_password, type) VALUES (?, ?, ?, ?, ?)",
        &user.id,
        &user.code,
        &user.name,
        &user.hashed_password,
        &user.type_,
    )
    .execute(conn)
    .await
    .unwrap();

    let conn = tx.acquire().await.unwrap();
    sqlx::query!(
        "INSERT INTO registrations (course_id, user_id) VALUES (?, ?)",
        &course_id,
        &user.id,
    )
    .execute(conn)
    .await
    .unwrap();

    let conn = tx.acquire().await.unwrap();
    let repo = RegistrationRepositoryInfra {};
    let got = repo
        .find_users_by_course_id(conn, &course_id)
        .await
        .unwrap();
    assert_eq!(got.len(), 1);
    let got = got.first().unwrap();
    assert_eq!(got, &user);
}
