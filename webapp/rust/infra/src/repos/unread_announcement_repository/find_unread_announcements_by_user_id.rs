use crate::repos::unread_announcement_repository::UnreadAnnouncementRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::announcement::Announcement;
use isucholar_core::models::course::Course;
use isucholar_core::models::user::UserID;
use isucholar_core::repos::unread_announcement_repository::UnreadAnnouncementRepository;
use sqlx::Acquire;

#[tokio::test]
async fn record_exist_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(conn)
        .await
        .unwrap();

    let repo = UnreadAnnouncementRepositoryInfra {};

    let course: Course = Faker.fake();
    let conn = tx.acquire().await.unwrap();
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
        .execute(conn)
        .await
        .unwrap();

    let user_id: UserID = Faker.fake();
    let conn = tx.acquire().await.unwrap();
    sqlx::query!(
        "INSERT INTO registrations (course_id, user_id) VALUES (?, ?)",
        &course.id,
        &user_id,
    )
    .execute(conn)
    .await
    .unwrap();

    let mut announcement: Announcement = Faker.fake();
    announcement.course_id = course.id.clone();
    let conn = tx.acquire().await.unwrap();
    sqlx::query!(
        "INSERT INTO announcements (id, course_id, title, message) VALUES (?, ?, ?, ?)",
        &announcement.id,
        &announcement.course_id,
        &announcement.title,
        &announcement.message,
    )
    .execute(conn)
    .await
    .unwrap();

    let conn = tx.acquire().await.unwrap();
    sqlx::query!(
        "INSERT INTO unread_announcements (announcement_id, user_id) VALUES (?, ?)",
        &announcement.id,
        &user_id,
    )
    .execute(conn)
    .await
    .unwrap();

    let announcements = repo
        .find_unread_announcements_by_user_id(&mut tx, &user_id, 10, 0, None)
        .await
        .unwrap();

    assert_eq!(announcements.len(), 1);
    let ann = announcements.first().unwrap();
    assert_eq!(ann.id, announcement.id);
    assert_eq!(ann.course_id, course.id);
    assert_eq!(ann.course_name, course.name);
    assert_eq!(ann.title, announcement.title);
    assert_eq!(ann.unread, true);
}

#[tokio::test]
async fn empty_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let user_id: UserID = Faker.fake();

    let repo = UnreadAnnouncementRepositoryInfra {};

    let announcements = repo
        .find_unread_announcements_by_user_id(conn, &user_id, 10, 0, None)
        .await
        .unwrap();

    assert_eq!(announcements.len(), 0);
}
