use crate::repos::registration_course_repository::RegistrationCourseRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::class::Class;
use isucholar_core::models::course::{Course, CourseID};
use isucholar_core::models::user::User;
use isucholar_core::repos::registration_course_repository::RegistrationCourseRepository;

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let course_id: CourseID = Faker.fake();

    let repo = RegistrationCourseRepositoryInfra {};
    let scores = repo
        .find_total_scores_by_course_id_group_by_user_id(&mut tx, &course_id)
        .await
        .unwrap();
    assert_eq!(scores.len(), 0);
}

#[tokio::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let course: Course = Faker.fake();

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

    let user: User = Faker.fake();
    sqlx::query!(
        "INSERT INTO users (id, code, name, hashed_password, type) VALUES (?, ?, ?, ?, ?)",
        &user.id,
        &user.code,
        &user.name,
        &user.hashed_password,
        &user.type_,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO registrations (course_id, user_id) VALUES (?, ?)",
        &course.id,
        &user.id,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let mut class: Class = Faker.fake();
    class.course_id = course.id.clone();
    sqlx::query!(
        "INSERT INTO classes (id, course_id, part, title, description, submission_closed) VALUES (?, ?, ?, ?, ?, ?)",
        &class.id,
        &class.course_id,
        &class.part,
        &class.title,
        &class.description,
        &class.submission_closed,
    ).execute(&mut tx).await.unwrap();

    sqlx::query!(
        "INSERT INTO submissions (user_id, class_id, file_name, score) VALUES (?, ?, ?, ?)",
        &user.id,
        &class.id,
        "file_name",
        100,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let course_id = CourseID::new(course.id.clone());

    let repo = RegistrationCourseRepositoryInfra {};
    let scores = repo
        .find_total_scores_by_course_id_group_by_user_id(&mut tx, &course_id)
        .await
        .unwrap();
    assert_eq!(scores.len(), 1);
    assert_eq!(scores.first().unwrap(), &100);
}
