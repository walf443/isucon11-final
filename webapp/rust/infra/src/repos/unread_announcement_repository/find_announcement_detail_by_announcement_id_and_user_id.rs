use crate::repos::unread_announcement_repository::UnreadAnnouncementRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::announcement::Announcement;
use isucholar_core::models::course::Course;
use isucholar_core::models::user::UserID;
use isucholar_core::repos::unread_announcement_repository::UnreadAnnouncementRepository;

#[tokio::test]
async fn record_exist_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let repo = UnreadAnnouncementRepositoryInfra {};

    let course: Course = Faker.fake();
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

    let mut announcement: Announcement = Faker.fake();
    announcement.course_id = course.id.clone();
    sqlx::query!(
        "INSERT INTO announcements (id, course_id, title, message) VALUES (?, ?, ?, ?)",
        &announcement.id,
        &announcement.course_id,
        &announcement.title,
        &announcement.message,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let user_id: UserID = Faker.fake();
    sqlx::query!(
        "INSERT INTO unread_announcements (announcement_id, user_id) VALUES (?, ?)",
        &announcement.id,
        &user_id,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let detail = repo
        .find_announcement_detail_by_announcement_id_and_user_id(
            &mut tx,
            &announcement.id,
            &user_id,
        )
        .await
        .unwrap()
        .unwrap();

    assert_eq!(detail.id, announcement.id);
    assert_eq!(detail.title, announcement.title);
    assert_eq!(detail.message, announcement.message);
    assert_eq!(detail.course_id, course.id);
}

#[tokio::test]
async fn none_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let repo = UnreadAnnouncementRepositoryInfra {};

    let course: Course = Faker.fake();
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

    let mut announcement: Announcement = Faker.fake();
    announcement.course_id = course.id.clone();
    sqlx::query!(
        "INSERT INTO announcements (id, course_id, title, message) VALUES (?, ?, ?, ?)",
        &announcement.id,
        &announcement.course_id,
        &announcement.title,
        &announcement.message,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let user_id: UserID = Faker.fake();

    let detail = repo
        .find_announcement_detail_by_announcement_id_and_user_id(
            &mut tx,
            &announcement.id,
            &user_id,
        )
        .await
        .unwrap();

    assert_eq!(detail.is_none(), true);
}
