use crate::repos::announcement_repository::AnnouncementRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::announcement::Announcement;
use isucholar_core::repos::announcement_repository::AnnouncementRepository;

#[tokio::test]
async fn success_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let repo = AnnouncementRepositoryInfra {};
    let input: Announcement = Faker.fake();
    repo.create(&mut tx, &input).await.unwrap();
    let got = repo.find_by_id(&mut tx, &input.id).await.unwrap();
    assert_eq!(got, input);
}

#[tokio::test]
#[should_panic(expected = "AnnouncementDuplicate")]
async fn duplicate_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query!("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();

    let repo = AnnouncementRepositoryInfra {};
    let input: Announcement = Faker.fake();
    repo.create(&mut tx, &input).await.unwrap();
    repo.create(&mut tx, &input).await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "SqlError")]
async fn other_error_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();

    let repo = AnnouncementRepositoryInfra {};
    let input: Announcement = Faker.fake();
    repo.create(&mut tx, &input).await.unwrap();
}
