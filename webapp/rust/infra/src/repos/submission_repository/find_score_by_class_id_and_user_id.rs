use crate::repos::submission_repository::SubmissionRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::submission::CreateSubmission;
use isucholar_core::models::user::UserID;
use isucholar_core::repos::submission_repository::SubmissionRepository;

#[tokio::test]
async fn exist_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let submission: CreateSubmission = Faker.fake();
    let repo = SubmissionRepositoryInfra {};

    sqlx::query!(
        "INSERT INTO submissions (user_id, class_id, file_name, score) VALUES (?, ?, ?, ?)",
        &submission.user_id,
        &submission.class_id,
        &submission.file_name,
        100,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let user_id = UserID::new(submission.user_id.clone());
    let score = repo
        .find_score_by_class_id_and_user_id(&mut tx, &submission.class_id, &user_id)
        .await
        .unwrap();
    assert_eq!(score.unwrap(), 100);
}

#[tokio::test]
async fn exist_but_null_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let submission: CreateSubmission = Faker.fake();
    let repo = SubmissionRepositoryInfra {};

    sqlx::query!(
        "INSERT INTO submissions (user_id, class_id, file_name) VALUES (?, ?, ?)",
        &submission.user_id,
        &submission.class_id,
        &submission.file_name,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let user_id = UserID::new(submission.user_id.clone());
    let score = repo
        .find_score_by_class_id_and_user_id(&mut tx, &submission.class_id, &user_id)
        .await
        .unwrap();
    assert_eq!(score.is_none(), true);
}

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let submission: CreateSubmission = Faker.fake();
    let repo = SubmissionRepositoryInfra {};

    let user_id = UserID::new(submission.user_id.clone());
    let score = repo
        .find_score_by_class_id_and_user_id(&mut tx, &submission.class_id, &user_id)
        .await
        .unwrap();
    assert_eq!(score.is_none(), true);
}
