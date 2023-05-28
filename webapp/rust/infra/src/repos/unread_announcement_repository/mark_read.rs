use crate::repos::unread_announcement_repository::UnreadAnnouncementRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::announcement::AnnouncementID;
use isucholar_core::models::user::UserID;
use isucholar_core::repos::unread_announcement_repository::UnreadAnnouncementRepository;
use sqlx::Acquire;

#[tokio::test]
async fn success_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(conn)
        .await
        .unwrap();

    let announcement_id: AnnouncementID = Faker.fake();
    let user_id: UserID = Faker.fake();
    let conn = tx.acquire().await.unwrap();

    sqlx::query!(
        "INSERT INTO unread_announcements (announcement_id, user_id) VALUES (?, ?)",
        announcement_id,
        user_id,
    )
    .execute(conn)
    .await
    .unwrap();

    let repo = UnreadAnnouncementRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();

    repo.mark_read(conn, &announcement_id, &user_id)
        .await
        .unwrap();

    let conn = tx.acquire().await.unwrap();
    let is_deleted: bool = sqlx::query_scalar!(
        "SELECT is_deleted as `is_deleted:bool` FROM unread_announcements WHERE announcement_id = ? AND user_id = ?",
        &announcement_id,
        &user_id,
    )
        .fetch_one(conn)
        .await
        .unwrap();
    assert!(is_deleted);
}
