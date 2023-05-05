use crate::models::user::{User, UserCode, UserID};
use crate::repos::user_repository::{HaveUserRepository, UserRepository};
use crate::services::error::Result;
use crate::services::HaveDBPool;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait UserService: Sync {
    async fn find_by_code(&self, code: &UserCode) -> Result<Option<User>>;
    async fn find_code_by_id(&self, user_id: &UserID) -> Result<Option<UserCode>>;
    fn verify_password(&self, user: &User, password: &str) -> Result<bool>;
}

pub trait HaveUserService {
    type Service: UserService;
    fn user_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait UserServiceImpl: Sync + HaveDBPool + HaveUserRepository {
    async fn find_by_code(&self, code: &UserCode) -> Result<Option<User>> {
        let pool = self.get_db_pool();
        let mut conn = pool.acquire().await?;

        let result = self.user_repo().find_by_code(&mut conn, &code).await?;
        Ok(result)
    }

    async fn find_code_by_id(&self, user_id: &UserID) -> Result<Option<UserCode>> {
        let pool = self.get_db_pool();
        let mut conn = pool.acquire().await?;

        let result = self
            .user_repo()
            .find_code_by_id(&mut conn, &user_id)
            .await?;

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
    async fn find_by_code(&self, code: &UserCode) -> Result<Option<User>> {
        UserServiceImpl::find_by_code(self, code).await
    }

    async fn find_code_by_id(&self, user_id: &UserID) -> Result<Option<UserCode>> {
        UserServiceImpl::find_code_by_id(self, user_id).await
    }

    fn verify_password(&self, user: &User, password: &str) -> Result<bool> {
        UserServiceImpl::verify_password(self, user, password)
    }
}
