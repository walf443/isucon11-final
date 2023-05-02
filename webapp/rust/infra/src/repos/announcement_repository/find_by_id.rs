use crate::repos::announcement_repository::AnnouncementRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::announcement::{Announcement, AnnouncementID};
use isucholar_core::repos::announcement_repository::AnnouncementRepository;

#[tokio::test]
#[should_panic(expected = "SqlError(RowNotFound)")]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let aid: AnnouncementID = Faker.fake();
    let repo = AnnouncementRepositoryInfra {};
    repo.find_by_id(&mut tx, &aid).await.unwrap();
}

#[tokio::test]
async fn success() {
    let db_pool = get_test_db_conn().await.unwrap();

    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let announcement: Announcement = Faker.fake();
    sqlx::query!(
        "INSERT INTO announcements (id, course_id, title, message) VALUES (?,?,?,?)",
        &announcement.id,
        &announcement.course_id,
        &announcement.title,
        &announcement.message,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let repo = AnnouncementRepositoryInfra {};
    let result = repo
        .find_by_id(&mut tx, &AnnouncementID::new(announcement.id.clone()))
        .await
        .unwrap();
    assert_eq!(result, announcement);
}
