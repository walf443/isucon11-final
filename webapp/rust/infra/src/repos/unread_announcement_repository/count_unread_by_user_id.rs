use crate::repos::unread_announcement_repository::UnreadAnnouncementRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
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
    let user_id: UserID = Faker.fake();
    let announcement_id1 = Faker.fake::<String>();
    let announcement_id2 = Faker.fake::<String>();
    let announcement_id3 = Faker.fake::<String>();
    let conn = tx.acquire().await.unwrap();

    sqlx::query!(
        "INSERT INTO unread_announcements (announcement_id, user_id, is_deleted) VALUES (?, ?, ?), (?, ?, ?), (?, ?, ?)",
        &announcement_id1,
        &user_id,
        false,
        &announcement_id2,
        &user_id,
        false,
        &announcement_id3,
        &user_id,
        true,
    )
    .execute(conn)
    .await
    .unwrap();

    let conn = tx.acquire().await.unwrap();
    let unread_count = repo.count_unread_by_user_id(conn, &user_id).await.unwrap();
    assert_eq!(unread_count, 2);
}

#[tokio::test]
async fn empty_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let repo = UnreadAnnouncementRepositoryInfra {};
    let user_id: UserID = Faker.fake();

    let unread_count = repo.count_unread_by_user_id(conn, &user_id).await.unwrap();
    assert_eq!(unread_count, 0);
}
