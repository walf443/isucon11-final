use crate::database::DBPool;
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_code_by_id(&self, pool: &DBPool, id: &str) -> Result<String>;
}

pub struct UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_code_by_id(&self, pool: &DBPool, id: &str) -> Result<String> {
        let user_code = sqlx::query_scalar("SELECT `code` FROM `users` WHERE `id` = ?")
            .bind(&id)
            .fetch_one(pool)
            .await?;

        Ok(user_code)
    }
}
