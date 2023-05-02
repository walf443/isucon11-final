use crate::repos::unread_announcement_repository::UnreadAnnouncementRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::announcement::AnnouncementID;
use isucholar_core::models::user::UserID;
use isucholar_core::repos::unread_announcement_repository::UnreadAnnouncementRepository;

#[tokio::test]
async fn success_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let announcement_id: AnnouncementID = Faker.fake();
    let user_id: UserID = Faker.fake();

    sqlx::query!(
        "INSERT INTO unread_announcements (announcement_id, user_id) VALUES (?, ?)",
        announcement_id,
        user_id,
    )
    .execute(&mut tx)
    .await
    .unwrap();

    let repo = UnreadAnnouncementRepositoryInfra {};

    repo.mark_read(&mut tx, &announcement_id, &user_id)
        .await
        .unwrap();

    let is_deleted: bool = sqlx::query_scalar!(
        "SELECT is_deleted as `is_deleted:bool` FROM unread_announcements WHERE announcement_id = ? AND user_id = ?",
        &announcement_id,
        &user_id,
    )
        .fetch_one(&mut tx)
        .await
        .unwrap();
    assert_eq!(is_deleted, true);
}
