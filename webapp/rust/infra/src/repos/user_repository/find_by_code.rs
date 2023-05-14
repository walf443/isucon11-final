use crate::repos::user_repository::UserRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::user::User;
use isucholar_core::repos::user_repository::UserRepository;
use sqlx::Acquire;

#[tokio::test]
async fn empty_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let user: User = Faker.fake();

    let repo = UserRepositoryInfra {};
    let got = repo.find_by_code(conn, &user.code).await.unwrap();
    assert_eq!(got.is_none(), true);
}

#[tokio::test]
async fn success_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    let conn = tx.acquire().await.unwrap();

    let mut user: User = Faker.fake();
    user.hashed_password.resize(60, 0);
    sqlx::query!(
        "INSERT INTO users (id, code, name, hashed_password, type) VALUES (?,?,?,?,?)",
        &user.id,
        &user.code,
        &user.name,
        &user.hashed_password,
        &user.type_
    )
    .execute(conn)
    .await
    .unwrap();

    let repo = UserRepositoryInfra {};
    let conn = tx.acquire().await.unwrap();
    let got = repo.find_by_code(conn, &user.code).await.unwrap().unwrap();

    assert_eq!(got, user);
}
