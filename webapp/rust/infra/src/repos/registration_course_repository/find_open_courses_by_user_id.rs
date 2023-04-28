use crate::repos::registration_course_repository::RegistrationCourseRepositoryInfra;
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use isucholar_core::repos::registration_course_repository::RegistrationCourseRepository;

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let repo = RegistrationCourseRepositoryInfra {};
    let users = repo
        .find_open_courses_by_user_id(&mut tx, "none_exist_user_id")
        .await
        .unwrap();
    assert_eq!(users.len(), 0)
}

#[tokio::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let closed_course = Course {
        id: "1".to_string(),
        code: "1".to_string(),
        type_: CourseType::LiberalArts,
        name: "".to_string(),
        description: "".to_string(),
        credit: 0,
        period: 0,
        day_of_week: DayOfWeek::Monday,
        teacher_id: "".to_string(),
        keywords: "".to_string(),
        status: CourseStatus::Closed,
    };

    sqlx::query!("INSERT INTO courses (id, code, type, name, description, credit, period, day_of_week, teacher_id, keywords, status) VALUES (?,?,?,?,?,?,?,?,?,?,?)",
        &closed_course.id,
        &closed_course.code,
        &closed_course.type_,
        &closed_course.name,
        &closed_course.description,
        &closed_course.credit,
        &closed_course.period,
        &closed_course.day_of_week,
        &closed_course.teacher_id,
        &closed_course.keywords,
        &closed_course.status,
    ).execute(&mut tx).await.unwrap();

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

    let user_id = "user_id";
    sqlx::query!(
        "INSERT INTO registrations (course_id, user_id) VALUES (?, ?), (?, ?)",
        &closed_course.id,
        &user_id,
        &course.id,
        &user_id,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let repo = RegistrationCourseRepositoryInfra {};
    let courses = repo
        .find_open_courses_by_user_id(&mut tx, &user_id)
        .await
        .unwrap();
    assert_eq!(courses.len(), 1);
    let got = courses.first().unwrap();
    assert_eq!(got, &course)
}
