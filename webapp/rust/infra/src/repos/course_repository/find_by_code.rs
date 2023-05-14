use crate::repos::course_repository::CourseRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::course::{Course, CourseCode};
use isucholar_core::repos::course_repository::CourseRepository;
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
    let course: Course = Faker.fake();

    let conn = tx.acquire().await.unwrap();
    sqlx::query!("INSERT INTO courses (id, code, type, name, description, credit, period, day_of_week, teacher_id, keywords, status) VALUES (?,?,?,?,?,?,?,?,?,?,?)",
        &course.id,
        &course.code,
        &course.type_,
        &course.name,
        &course.description,
        &course.credit,
        &course.period,
        &course.day_of_week,
        &course.teacher_id,
        &course.keywords,
        &course.status,
    ).execute(conn).await.unwrap();

    let repo = CourseRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    let got = repo.find_by_code(conn, &course.code).await.unwrap();
    assert_eq!(got, course)
}

#[tokio::test]
#[should_panic(expected = "SqlError(RowNotFound)")]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let code: CourseCode = Faker.fake();
    let repo = CourseRepositoryInfra {};
    repo.find_by_code(conn, &code).await.unwrap();
}
