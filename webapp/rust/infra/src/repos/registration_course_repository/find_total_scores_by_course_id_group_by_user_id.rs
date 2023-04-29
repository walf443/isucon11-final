use crate::repos::registration_course_repository::RegistrationCourseRepositoryInfra;
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::class::Class;
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use isucholar_core::models::user::User;
use isucholar_core::models::user_type::UserType;
use isucholar_core::repos::registration_course_repository::RegistrationCourseRepository;

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let repo = RegistrationCourseRepositoryInfra {};
    let scores = repo
        .find_total_scores_by_course_id_group_by_user_id(&mut tx, "none_exist_course_id")
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

    let course = Course {
        id: "2".to_string(),
        code: "2".to_string(),
        type_: CourseType::LiberalArts,
        name: "".to_string(),
        description: "".to_string(),
        credit: 0,
        period: 0,
        day_of_week: DayOfWeek::Monday,
        teacher_id: "".to_string(),
        keywords: "".to_string(),
        status: CourseStatus::Registration,
    };

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

    let user = User {
        id: "user_id".to_string(),
        code: "code".to_string(),
        name: "name".to_string(),
        hashed_password: vec![],
        type_: UserType::Student,
    };
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

    let class = Class {
        id: "1".to_string(),
        course_id: course.id.clone(),
        part: 1,
        title: "title".to_string(),
        description: "description".to_string(),
        submission_closed: false,
    };
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

    let repo = RegistrationCourseRepositoryInfra {};
    let scores = repo
        .find_total_scores_by_course_id_group_by_user_id(&mut tx, &course.id)
        .await
        .unwrap();
    assert_eq!(scores.len(), 1);
    assert_eq!(scores.first().unwrap(), &100);
}
