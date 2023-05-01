use crate::db::{DBConn, DBPool};
use crate::models::user::User;
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find(&self, conn: &mut DBConn, id: &str) -> Result<User>;
    async fn find_by_code(&self, conn: &mut DBConn, code: &str) -> Result<Option<User>>;
    async fn find_code_by_id(&self, pool: &DBPool, id: &str) -> Result<Option<String>>;
    async fn find_gpas_group_by_user_id(&self, pool: &DBPool) -> Result<Vec<f64>>;
}

pub trait HaveUserRepository {
    type Repo: UserRepository + Sync;
    fn user_repo(&self) -> &Self::Repo;
}
