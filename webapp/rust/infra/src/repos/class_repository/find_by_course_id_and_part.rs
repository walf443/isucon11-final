use crate::repos::class_repository::ClassRepositoryInfra;
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::class::Class;
use isucholar_core::repos::class_repository::ClassRepository;

#[tokio::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();
    let class = Class {
        id: "1".to_string(),
        course_id: "course_id".to_string(),
        part: 2,
        title: "title".to_string(),
        description: "description".to_string(),
        submission_closed: true,
    };
    sqlx::query!("INSERT INTO classes (id, course_id, part, title, description, submission_closed) VALUES (?,?,?,?,?,?)",
        &class.id,
        &class.course_id,
        &class.part,
        &class.title,
        &class.description,
        &class.submission_closed,
    ).execute(&mut tx).await.unwrap();

    let repo = ClassRepositoryInfra {};
    let got = repo
        .find_by_course_id_and_part(&mut tx, &class.course_id, &class.part)
        .await
        .unwrap();
    assert_eq!(got, class)
}

#[tokio::test]
#[should_panic(expected = "RowNotFound")]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let repo = ClassRepositoryInfra {};
    repo.find_by_course_id_and_part(&mut tx, "none_exist_course_id", &0)
        .await
        .unwrap();
}