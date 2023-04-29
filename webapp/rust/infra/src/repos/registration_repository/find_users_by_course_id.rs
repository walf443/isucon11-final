use crate::repos::registration_repository::RegistrationRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::user::User;
use isucholar_core::repos::registration_repository::RegistrationRepository;

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let repo = RegistrationRepositoryInfra {};
    let got = repo
        .find_users_by_course_id(&mut tx, "none_exist_course_id")
        .await
        .unwrap();
    assert_eq!(got.len(), 0);
}

#[tokio::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let course_id = Faker.fake::<String>();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();
    let mut user: User = Faker.fake();
    user.hashed_password.resize(60, 0);

    sqlx::query!(
        "INSERT INTO users (id, code, name, hashed_password, type) VALUES (?, ?, ?, ?, ?)",
        &user.id,
        &user.code,
        &user.name,
        &user.hashed_password,
        &user.type_,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO registrations (course_id, user_id) VALUES (?, ?)",
        &course_id,
        &user.id,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let repo = RegistrationRepositoryInfra {};
    let got = repo
        .find_users_by_course_id(&mut tx, &course_id)
        .await
        .unwrap();
    assert_eq!(got.len(), 1);
    let got = got.first().unwrap();
    assert_eq!(got, &user);
}
