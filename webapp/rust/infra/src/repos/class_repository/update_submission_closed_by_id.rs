use crate::repos::class_repository::ClassRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::class::{Class, ClassID};
use isucholar_core::repos::class_repository::ClassRepository;

#[tokio::test]
async fn success_case() {
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
    repo.update_submission_closed_by_id(&mut tx, &ClassID::new(class.id.clone()))
        .await
        .unwrap();

    let got = sqlx::query_scalar!(
        "SELECT submission_closed AS `submission_closed:bool` FROM classes WHERE id = ?",
        &class.id
    )
    .fetch_one(&mut tx)
    .await
    .unwrap();
    assert_eq!(got, true);
}

#[tokio::test]
async fn specify_none_exist_id_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let repo = ClassRepositoryInfra {};
    let class_id: ClassID = Faker.fake();
    repo.update_submission_closed_by_id(&mut tx, &class_id)
        .await
        .unwrap();
}
