use crate::repos::submission_repository::SubmissionRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::submission::CreateSubmission;
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

    let submission: CreateSubmission = Faker.fake();
    let repo = SubmissionRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    repo.create_or_update(conn, &submission).await.unwrap();

    let conn = tx.acquire().await.unwrap();
    let row_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM submissions WHERE user_id = ? AND class_id = ?",
        submission.user_id,
        submission.class_id,
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

    let mut submission: CreateSubmission = Faker.fake();
    let repo = SubmissionRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    repo.create_or_update(conn, &submission).await.unwrap();
    submission.file_name = Faker.fake::<String>();
    let conn = tx.acquire().await.unwrap();
    repo.create_or_update(conn, &submission).await.unwrap();

    let conn = tx.acquire().await.unwrap();
    let file_name = sqlx::query_scalar!(
        "SELECT file_name FROM submissions WHERE user_id = ? AND class_id = ?",
        submission.user_id,
        submission.class_id,
    )
    .fetch_one(conn)
    .await
    .unwrap();

    assert_eq!(file_name, submission.file_name);
}
