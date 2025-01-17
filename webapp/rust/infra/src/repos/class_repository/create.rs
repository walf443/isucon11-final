use crate::repos::class_repository::ClassRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::class::{Class, ClassID, CreateClass};
use isucholar_core::models::course::CourseID;
use isucholar_core::repos::class_repository::ClassRepository;
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

    let class_id: ClassID = Faker.fake();
    let class: CreateClass = Faker.fake();

    let repo = ClassRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    repo.create(conn, &class_id, &class).await.unwrap();

    let conn = tx.acquire().await.unwrap();
    let got = sqlx::query_as!(Class,
        "SELECT id as `id:ClassID`, course_id as `course_id:CourseID`, part, title, description, submission_closed as `submission_closed:bool` FROM classes WHERE id = ?",
        &class_id)
        .fetch_one(conn)
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
    let conn = tx.acquire().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(conn)
        .await
        .unwrap();

    let class_id: ClassID = Faker.fake();
    let class: CreateClass = Faker.fake();

    let repo = ClassRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    repo.create(conn, &class_id, &class).await.unwrap();
    let conn = tx.acquire().await.unwrap();
    repo.create(conn, &class_id, &class).await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "SqlError")]
async fn error_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let class_id: ClassID = Faker.fake();
    let class: CreateClass = Faker.fake();

    let repo = ClassRepositoryInfra {};
    repo.create(conn, &class_id, &class).await.unwrap();
}
