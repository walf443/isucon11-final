use crate::repos::unread_announcement_repository::UnreadAnnouncementRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::repos::unread_announcement_repository::UnreadAnnouncementRepository;

#[tokio::test]
async fn success_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let repo = UnreadAnnouncementRepositoryInfra {};
    let announcement_id = Faker.fake::<String>();
    let user_id = Faker.fake::<String>();

    repo.create(&mut tx, &announcement_id, &user_id)
        .await
        .unwrap();

    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM unread_announcements WHERE announcement_id = ? AND user_id = ?",
        &announcement_id,
        &user_id,
    )
    .fetch_one(&mut tx)
    .await
    .unwrap();
    assert_eq!(count, 1);
}
