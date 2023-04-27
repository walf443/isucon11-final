use crate::repos::course_repository::CourseRepositoryInfra;
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use isucholar_core::repos::course_repository::CourseRepository;

#[tokio::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();
    let code = "code";
    let course = Course {
        id: "1".to_string(),
        code: code.to_string(),
        type_: CourseType::LiberalArts,
        name: "name".to_string(),
        description: "description".to_string(),
        credit: 1,
        period: 2,
        day_of_week: DayOfWeek::Monday,
        teacher_id: "teacher_id".to_string(),
        keywords: "keywords".to_string(),
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

    let repo = CourseRepositoryInfra {};
    repo.update_status_by_id(&mut tx, &course.id, &CourseStatus::Closed)
        .await
        .unwrap();
    let status = sqlx::query_scalar!(
        "SELECT status AS `status:CourseStatus` FROM courses WHERE id = ?",
        &course.id
    )
    .fetch_one(&mut tx)
    .await
    .unwrap();
    assert_eq!(status, CourseStatus::Closed)
}

#[tokio::test]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let repo = CourseRepositoryInfra {};
    let result = repo
        .update_status_by_id(&mut tx, "none_exist_course_id", &CourseStatus::Closed)
        .await;
    assert_eq!(result.is_ok(), true);
}
