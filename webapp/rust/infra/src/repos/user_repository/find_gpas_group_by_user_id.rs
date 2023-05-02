use crate::repos::user_repository::UserRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::class::Class;
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus::Closed;
use isucholar_core::models::submission::CreateSubmission;
use isucholar_core::models::user::User;
use isucholar_core::models::user_type::UserType::Student;
use isucholar_core::repos::user_repository::UserRepository;

#[tokio::test]
async fn empty_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();

    let repo = UserRepositoryInfra {};
    let gpas = repo.find_gpas_group_by_user_id(&mut tx).await.unwrap();
    assert_eq!(gpas.len(), 0);
}

#[tokio::test]
async fn have_record_without_submissions_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let mut user: User = Faker.fake();
    user.type_ = Student;
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

    let mut course: Course = Faker.fake();
    course.status = Closed;
    sqlx::query!(
        "INSERT INTO courses (id, code, type, name, description, credit, period, day_of_week, teacher_id, keywords, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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

    let repo = UserRepositoryInfra {};
    let gpas = repo.find_gpas_group_by_user_id(&mut tx).await.unwrap();
    assert_eq!(gpas.len(), 1);
    let gpa = gpas.first().unwrap();
    assert_eq!(gpa, &0.0);
}

#[tokio::test]
async fn have_record_with_submissions_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let mut user: User = Faker.fake();
    user.type_ = Student;
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

    let mut course: Course = Faker.fake();
    course.status = Closed;
    sqlx::query!(
        "INSERT INTO courses (id, code, type, name, description, credit, period, day_of_week, teacher_id, keywords, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
    )
        .execute(&mut tx)
        .await
        .unwrap();

    let mut submission: CreateSubmission = Faker.fake();
    submission.class_id = class.id.clone().to_string();
    submission.user_id = user.id.clone();

    let score = 100;
    sqlx::query!(
        "INSERT INTO submissions (user_id, class_id, file_name, score) VALUES (?, ?, ?, ?)",
        &submission.user_id,
        &submission.class_id,
        &submission.file_name,
        score,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let repo = UserRepositoryInfra {};
    let gpas = repo.find_gpas_group_by_user_id(&mut tx).await.unwrap();
    assert_eq!(gpas.len(), 1);
    let gpa = gpas.first().unwrap();
    assert_eq!(gpa, &1.0);
}
