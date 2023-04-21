use crate::models::user::User;
use crate::repos::user_repository::{HaveUserRepository, UserRepository};
use crate::services::error::Result;
use crate::services::HaveDBPool;
use async_trait::async_trait;

#[async_trait]
pub trait UserService: Sync {
    async fn find_by_code(&self, code: &str) -> Result<Option<User>>;
    async fn find_code_by_id(&self, user_id: &str) -> Result<Option<String>>;
    async fn find_gpas_group_by_user_id(&self) -> Result<Vec<f64>>;
    fn verify_password(&self, user: &User, password: &str) -> Result<bool>;
}

pub trait HaveUserService {
    type Service: UserService;
    fn user_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait UserServiceImpl: Sync + HaveDBPool + HaveUserRepository {
    async fn find_by_code(&self, code: &str) -> Result<Option<User>> {
        let pool = self.get_db_pool();
        let result = self.user_repo().find_by_code(pool, code).await?;
        Ok(result)
    }

    async fn find_code_by_id(&self, user_id: &str) -> Result<Option<String>> {
        let pool = self.get_db_pool();
        let result = self.user_repo().find_code_by_id(pool, user_id).await?;
        Ok(result)
    }

    async fn find_gpas_group_by_user_id(&self) -> Result<Vec<f64>> {
        let pool = self.get_db_pool();
        let result = self.user_repo().find_gpas_group_by_user_id(pool).await?;
        Ok(result)
    }

    fn verify_password(&self, user: &User, password: &str) -> Result<bool> {
        if !bcrypt::verify(
            password,
            &String::from_utf8(user.hashed_password.clone()).unwrap(),
        )? {
            return Ok(false);
        }

        Ok(true)
    }
}

#[async_trait]
impl<S: UserServiceImpl> UserService for S {
    async fn find_by_code(&self, code: &str) -> Result<Option<User>> {
        UserServiceImpl::find_by_code(self, code).await
    }

    async fn find_code_by_id(&self, user_id: &str) -> Result<Option<String>> {
        UserServiceImpl::find_code_by_id(self, user_id).await
    }

    async fn find_gpas_group_by_user_id(&self) -> Result<Vec<f64>> {
        UserServiceImpl::find_gpas_group_by_user_id(self).await
    }

    fn verify_password(&self, user: &User, password: &str) -> Result<bool> {
        UserServiceImpl::verify_password(self, user, password)
    }
}
