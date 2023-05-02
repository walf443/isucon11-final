use crate::repos::submission_repository::SubmissionRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::class::ClassID;
use isucholar_core::models::submission::CreateSubmission;
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
        "INSERT INTO submissions (user_id, class_id, file_name) VALUES (?, ?, ?)",
        &submission.user_id,
        &submission.class_id,
        &submission.file_name,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let class_id = ClassID::new(submission.class_id.clone());

    let count = repo.count_by_class_id(&mut tx, &class_id).await.unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let submission: CreateSubmission = Faker.fake();
    let repo = SubmissionRepositoryInfra {};

    let class_id = ClassID::new(submission.class_id.clone());
    let count = repo.count_by_class_id(&mut tx, &class_id).await.unwrap();
    assert_eq!(count, 0);
}
