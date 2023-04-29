use crate::repos::registration_repository::RegistrationRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::repos::registration_repository::RegistrationRepository;

#[tokio::test]
async fn false_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let repo = RegistrationRepositoryInfra {};
    let got = repo
        .exist_by_user_id_and_course_id(&mut tx, "none_exist_user_id", "none_exist_course_id")
        .await
        .unwrap();
    assert_eq!(got, false);
}

#[tokio::test]
async fn true_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let user_id = Faker.fake::<String>();
    let course_id = Faker.fake::<String>();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();
    sqlx::query!(
        "INSERT INTO registrations (course_id, user_id) VALUES (?, ?)",
        &course_id,
        &user_id,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let repo = RegistrationRepositoryInfra {};
    let got = repo
        .exist_by_user_id_and_course_id(&mut tx, &user_id, &course_id)
        .await
        .unwrap();
    assert_eq!(got, true);
}
