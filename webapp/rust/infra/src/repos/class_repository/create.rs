use crate::repos::class_repository::ClassRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::class::{Class, CreateClass};
use isucholar_core::repos::class_repository::ClassRepository;

#[tokio::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let class: CreateClass = Faker.fake();

    let repo = ClassRepositoryInfra {};
    repo.create(&mut tx, &class).await.unwrap();

    let got = sqlx::query_as!(Class,
        "SELECT id, course_id, part, title, description, submission_closed as `submission_closed:bool` FROM classes WHERE id = ?",
        &class.id)
        .fetch_one(&mut tx)
        .await
        .unwrap();

    assert_eq!(got.course_id, class.course_id);
    assert_eq!(got.part, class.part);
    assert_eq!(got.title, class.title);
    assert_eq!(got.description, class.description);
}

#[tokio::test]
#[should_panic(expected = "ClassDuplicate")]
async fn duplicate_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let class: CreateClass = Faker.fake();

    let repo = ClassRepositoryInfra {};
    repo.create(&mut tx, &class).await.unwrap();
    repo.create(&mut tx, &class).await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "SqlError")]
async fn error_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let class: CreateClass = Faker.fake();

    let repo = ClassRepositoryInfra {};
    repo.create(&mut tx, &class).await.unwrap();
}
