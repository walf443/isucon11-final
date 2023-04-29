use crate::repos::class_repository::ClassRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::class::Class;
use isucholar_core::repos::class_repository::ClassRepository;

#[tokio::test]
async fn true_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let mut class: Class = Faker.fake();
    class.submission_closed = true;

    sqlx::query!("INSERT INTO classes (id, course_id, part, title, description, submission_closed) VALUES (?,?,?,?,?,?)",
        &class.id,
        &class.course_id,
        &class.part,
        &class.title,
        &class.description,
        &class.submission_closed,
    ).execute(&mut tx).await.unwrap();

    let repo = ClassRepositoryInfra {};
    let result = repo
        .find_submission_closed_by_id_with_shared_lock(&mut tx, &class.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(result, true)
}

#[tokio::test]
async fn false_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let mut class: Class = Faker.fake();
    class.submission_closed = false;

    sqlx::query!("INSERT INTO classes (id, course_id, part, title, description, submission_closed) VALUES (?,?,?,?,?,?)",
        &class.id,
        &class.course_id,
        &class.part,
        &class.title,
        &class.description,
        &class.submission_closed,
    ).execute(&mut tx).await.unwrap();

    let repo = ClassRepositoryInfra {};
    let result = repo
        .find_submission_closed_by_id_with_shared_lock(&mut tx, &class.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(result, false)
}

#[tokio::test]
async fn none_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let repo = ClassRepositoryInfra {};
    let result = repo
        .find_submission_closed_by_id_with_shared_lock(&mut tx, "none_exist_class_id")
        .await
        .unwrap();
    assert_eq!(result.is_none(), true);
}
