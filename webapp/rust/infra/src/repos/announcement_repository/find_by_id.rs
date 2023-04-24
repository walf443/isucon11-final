use crate::repos::announcement_repository::AnnouncementRepositoryInfra;
use isucholar_core::db::get_test_db_conn;
use isucholar_core::repos::announcement_repository::AnnouncementRepository;

#[tokio::test]
#[should_panic(expected = "SqlError(RowNotFound)")]
async fn empty_case() {
    let db_pool = get_test_db_conn().await.unwrap();
    let mut tx = db_pool.begin().await.unwrap();
    sqlx::query("SELECT GET_LOCK('test', -1)")
        .execute(&mut tx)
        .await
        .unwrap();

    let repo = AnnouncementRepositoryInfra {};
    repo.find_by_id(&db_pool, "1").await.unwrap();

    sqlx::query("SELECT RELEASE_LOCK('test')")
        .execute(&mut tx)
        .await
        .unwrap();
}

#[tokio::test]
async fn success() {
    let db_pool = get_test_db_conn().await.unwrap();

    let mut tx2 = db_pool.begin().await.unwrap();
    sqlx::query("SELECT GET_LOCK('test', -1)")
        .execute(&mut tx2)
        .await
        .unwrap();

    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();
    sqlx::query("TRUNCATE TABLE announcements")
        .execute(&mut tx)
        .await
        .unwrap();
    sqlx::query("INSERT INTO announcements (id, course_id, title, message) VALUES (?,?,?,?)")
        .bind("1")
        .bind("course_id")
        .bind("title")
        .bind("message")
        .execute(&mut tx)
        .await
        .unwrap();

    sqlx::query("SET foreign_key_checks=1")
        .execute(&mut tx)
        .await
        .unwrap();

    tx.commit().await.unwrap();

    let repo = AnnouncementRepositoryInfra {};
    let result = repo.find_by_id(&db_pool, "1").await.unwrap();
    assert_eq!(result.id, "1");

    let mut tx = db_pool.begin().await.unwrap();

    sqlx::query("SET foreign_key_checks=0")
        .execute(&mut tx)
        .await
        .unwrap();
    sqlx::query("TRUNCATE TABLE announcements")
        .execute(&mut tx)
        .await
        .unwrap();
    sqlx::query("SET foreign_key_checks=1")
        .execute(&mut tx)
        .await
        .unwrap();

    tx.commit().await.unwrap();

    sqlx::query("SELECT RELEASE_LOCK('test')")
        .execute(&mut tx2)
        .await
        .unwrap();
}
