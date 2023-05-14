use crate::repos::submission_repository::SubmissionRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::submission::CreateSubmission;
use isucholar_core::models::user::User;
use isucholar_core::repos::submission_repository::SubmissionRepository;
use sqlx::Acquire;

#[tokio::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(conn)
        .await
        .unwrap();

    let user: User = Faker.fake();
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

    let mut submission: CreateSubmission = Faker.fake();
    submission.user_id = user.id.clone();
    let conn = tx.acquire().await.unwrap();

    sqlx::query!(
        "INSERT INTO submissions (user_id, class_id, file_name) VALUES (?, ?, ?)",
        &submission.user_id,
        &submission.class_id,
        &submission.file_name,
    )
    .execute(conn)
    .await
    .unwrap();

    let repo = SubmissionRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();

    let submissions = repo
        .find_all_with_user_code_by_class_id(conn, &submission.class_id)
        .await
        .unwrap();
    assert_eq!(submissions.len(), 1);
    let got = submissions.first().unwrap();
    assert_eq!(got.file_name, submission.file_name);
    assert_eq!(got.user_id, submission.user_id);
    assert_eq!(got.user_code, user.code);
}

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let submission: CreateSubmission = Faker.fake();

    let repo = SubmissionRepositoryInfra {};

    let submissions = repo
        .find_all_with_user_code_by_class_id(conn, &submission.class_id)
        .await
        .unwrap();
    assert_eq!(submissions.len(), 0);
}
