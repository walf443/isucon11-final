use crate::repos::submission_repository::SubmissionRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::submission::CreateSubmission;
use isucholar_core::models::user::User;
use isucholar_core::repos::submission_repository::SubmissionRepository;
use num_bigint::BigInt;
use num_bigint::Sign::Plus;
use sqlx::types::BigDecimal;
use sqlx::Acquire;

#[tokio::test]
async fn exist_case() {
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

    let repo = SubmissionRepositoryInfra {};
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

    let conn = tx.acquire().await.unwrap();
    repo.update_score_by_user_code_and_class_id(conn, &user.code, &submission.class_id, 100)
        .await
        .unwrap();

    let conn = tx.acquire().await.unwrap();
    let score = sqlx::query_scalar!(
        "SELECT SUM(score) FROM submissions WHERE user_id = ? AND class_id = ?",
        submission.user_id,
        submission.class_id,
    )
    .fetch_one(conn)
    .await
    .unwrap()
    .unwrap();

    assert_eq!(score, BigDecimal::new(BigInt::new(Plus, vec![100]), 0));
}
