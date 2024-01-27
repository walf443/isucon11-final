use crate::repos::class_repository::ClassRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::class::Class;
use isucholar_core::models::course::CourseID;
use isucholar_core::models::user::UserID;
use isucholar_core::repos::class_repository::ClassRepository;
use sqlx::Acquire;

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let repo = ClassRepositoryInfra {};
    let user_id: UserID = Faker.fake();
    let course_id: CourseID = Faker.fake();
    let got = repo
        .find_all_with_submitted_by_user_id_and_course_id(conn, &user_id, &course_id)
        .await
        .unwrap();
    assert_eq!(got.len(), 0);
}

#[tokio::test]
async fn without_submission_record_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(conn)
        .await
        .unwrap();
    let class: Class = Faker.fake();
    let conn = tx.acquire().await.unwrap();
    sqlx::query!("INSERT INTO classes (id, course_id, part, title, description, submission_closed) VALUES (?,?,?,?,?,?)",
        &class.id,
        &class.course_id,
        &class.part,
        &class.title,
        &class.description,
        &class.submission_closed,
    ).execute(conn).await.unwrap();

    let user_id: UserID = Faker.fake();
    let repo = ClassRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    let got = repo
        .find_all_with_submitted_by_user_id_and_course_id(conn, &user_id, &class.course_id)
        .await
        .unwrap();
    assert_eq!(got.len(), 1);

    let got = got.first().unwrap();
    assert_eq!(got.id, class.id);
    assert_eq!(got.course_id, class.course_id);
    assert_eq!(got.part, class.part);
    assert_eq!(got.title, class.title);
    assert_eq!(got.description, class.description);
    assert_eq!(got.submission_closed, class.submission_closed);
    assert!(!got.submitted)
}

#[tokio::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(conn)
        .await
        .unwrap();
    let class: Class = Faker.fake();
    let conn = tx.acquire().await.unwrap();
    sqlx::query!("INSERT INTO classes (id, course_id, part, title, description, submission_closed) VALUES (?,?,?,?,?,?)",
        &class.id,
        &class.course_id,
        &class.part,
        &class.title,
        &class.description,
        &class.submission_closed,
    ).execute(conn).await.unwrap();

    let user_id: UserID = Faker.fake();
    let conn = tx.acquire().await.unwrap();
    sqlx::query!(
        "INSERT INTO submissions (user_id,class_id,file_name,score) VALUES (?,?,?,?)",
        &user_id,
        &class.id,
        "file_name",
        0,
    )
    .execute(conn)
    .await
    .unwrap();

    let repo = ClassRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    let got = repo
        .find_all_with_submitted_by_user_id_and_course_id(conn, &user_id, &class.course_id)
        .await
        .unwrap();
    assert_eq!(got.len(), 1);

    let got = got.first().unwrap();
    assert_eq!(got.id, class.id);
    assert_eq!(got.course_id, class.course_id);
    assert_eq!(got.part, class.part);
    assert_eq!(got.title, class.title);
    assert_eq!(got.description, class.description);
    assert_eq!(got.submission_closed, class.submission_closed);
    assert!(got.submitted)
}
