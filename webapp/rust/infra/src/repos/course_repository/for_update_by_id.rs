use crate::repos::course_repository::CourseRepositoryInfra;
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use isucholar_core::repos::course_repository::CourseRepository;

#[tokio::test]
async fn true_case() {
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
    let got = repo.for_update_by_id(&mut tx, &course.id).await.unwrap();
    assert_eq!(got, true)
}

#[tokio::test]
async fn false_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let repo = CourseRepositoryInfra {};
    let got = repo
        .for_update_by_id(&mut tx, "none_exist_course_id")
        .await
        .unwrap();
    assert_eq!(got, false)
}
