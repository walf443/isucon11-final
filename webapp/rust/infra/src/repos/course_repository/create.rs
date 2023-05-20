use crate::repos::course_repository::CourseRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::course::{Course, CourseCode, CourseID, CreateCourse};
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use isucholar_core::models::user::UserID;
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

    let course: CreateCourse = Faker.fake();

    let repo = CourseRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    let course_id = repo.create(conn, &course).await.unwrap();

    assert_eq!(course_id, course.id);

    let conn = tx.acquire().await.unwrap();
    let got: Course = sqlx::query_as!(
        Course,
        r"
            SELECT
               courses.id as `id:CourseID`,
               courses.code as `code:CourseCode`,
               courses.type as `type_:CourseType`,
               courses.name,
               description,
               credit,
               period,
               day_of_week as `day_of_week:DayOfWeek`,
               teacher_id as `teacher_id:UserID`,
               keywords,
               status as `status:CourseStatus`
            FROM courses WHERE id = ?",
        &course_id
    )
    .fetch_one(conn)
    .await
    .unwrap();

    assert_eq!(got.id, course.id);
    assert_eq!(got.code, course.code);
    assert_eq!(got.type_, course.type_);
    assert_eq!(got.name, course.name);
    assert_eq!(got.description, course.description);
    assert_eq!(got.period, course.period);
    assert_eq!(got.day_of_week, course.day_of_week);
    assert_eq!(got.keywords, course.keywords);
    assert_eq!(got.status, CourseStatus::Registration);
    assert_eq!(got.teacher_id, course.user_id);
}

#[tokio::test]
#[should_panic(expected = "CourseDuplicate")]
async fn duplicate_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(conn)
        .await
        .unwrap();

    let course: CreateCourse = Faker.fake();

    let repo = CourseRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    repo.create(conn, &course).await.unwrap();
    let conn = tx.acquire().await.unwrap();
    repo.create(conn, &course).await.unwrap();
}
