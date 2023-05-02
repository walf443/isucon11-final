use crate::repos::course_repository::CourseRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::course::{Course, CourseID};
use isucholar_core::models::user::User;
use isucholar_core::models::user_type::UserType::Teacher;
use isucholar_core::repos::course_repository::CourseRepository;

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let course_id: CourseID = Faker.fake();

    let repo = CourseRepositoryInfra {};
    let got = repo
        .find_with_teacher_by_id(&mut tx, &course_id)
        .await
        .unwrap();
    assert_eq!(got.is_none(), true);
}

#[tokio::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    let mut teacher: User = Faker.fake();
    teacher.type_ = Teacher;

    sqlx::query!(
        "INSERT INTO users (id, code, name, hashed_password, type) VALUES (?,?,?,?,?)",
        &teacher.id,
        &teacher.code,
        &teacher.name,
        &teacher.hashed_password,
        &teacher.type_,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let mut course: Course = Faker.fake();
    course.teacher_id = teacher.id.clone();

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
    ).execute(&mut tx).await.unwrap();

    let repo = CourseRepositoryInfra {};
    let got = repo
        .find_with_teacher_by_id(&mut tx, &CourseID::new(course.id.clone()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(got.id, course.id);
    assert_eq!(got.code, course.code);
    assert_eq!(got.name, course.name);
    assert_eq!(got.description, course.description);
    assert_eq!(got.credit, course.credit);
    assert_eq!(got.period, course.period);
    assert_eq!(got.day_of_week, course.day_of_week);
    assert_eq!(got.teacher_id, course.teacher_id);
    assert_eq!(got.keywords, course.keywords);
    assert_eq!(got.status, course.status);
    assert_eq!(got.teacher, teacher.name);
}
