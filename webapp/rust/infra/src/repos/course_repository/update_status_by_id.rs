use crate::repos::course_repository::CourseRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::course::{Course, CourseID};
use isucholar_core::models::course_status::CourseStatus;
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
    let mut course: Course = Faker.fake();
    course.status = CourseStatus::Registration;

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
    repo.update_status_by_id(conn, &course.id, &CourseStatus::Closed)
        .await
        .unwrap();

    let conn = tx.acquire().await.unwrap();
    let status = sqlx::query_scalar!(
        "SELECT status AS `status:CourseStatus` FROM courses WHERE id = ?",
        &course.id
    )
    .fetch_one(conn)
    .await
    .unwrap();
    assert_eq!(status, CourseStatus::Closed)
}

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let course_id: CourseID = Faker.fake();

    let repo = CourseRepositoryInfra {};
    let result = repo
        .update_status_by_id(conn, &course_id, &CourseStatus::Closed)
        .await;
    assert_eq!(result.is_ok(), true);
}
