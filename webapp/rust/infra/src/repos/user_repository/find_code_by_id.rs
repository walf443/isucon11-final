use crate::repos::user_repository::UserRepositoryInfra;
use fake::{Fake, Faker};
use isucholar_core::db::get_test_db_conn;
use isucholar_core::models::user::{User, UserCode, UserID};
use isucholar_core::repos::user_repository::UserRepository;

#[tokio::test]
async fn empty_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();

    let user: User = Faker.fake();

    let repo = UserRepositoryInfra {};
    let got = repo
        .find_code_by_id(&mut tx, &UserID::new(user.id.clone()))
        .await
        .unwrap();
    assert_eq!(got.is_none(), true);
}

#[tokio::test]
async fn success_case() {
    let pool = get_test_db_conn().await.unwrap();
    let mut tx = pool.begin().await.unwrap();

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
    .execute(&mut tx)
    .await
    .unwrap();

    let repo = UserRepositoryInfra {};
    let got = repo
        .find_code_by_id(&mut tx, &UserID::new(user.id.clone()))
        .await
        .unwrap()
        .unwrap();

    assert_eq!(got, UserCode::new(user.code.clone()));
}
