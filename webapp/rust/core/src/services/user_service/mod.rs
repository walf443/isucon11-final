use crate::models::user::User;
use crate::repos::user_repository::{HaveUserRepository, UserRepository};
use crate::services::error::Error::InvalidPassword;
use crate::services::error::Result;
use crate::services::HaveDBPool;
use async_trait::async_trait;

#[async_trait]
pub trait UserService: Sync {
    async fn find_by_code(&self, code: &str) -> Result<Option<User>>;
    fn verify_password(&self, user: &User, password: &str) -> Result<()>;
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

    fn verify_password(&self, user: &User, password: &str) -> Result<()> {
        if !bcrypt::verify(
            password,
            &String::from_utf8(user.hashed_password.clone()).unwrap(),
        )? {
            return Err(InvalidPassword);
        }

        Ok(())
    }
}

#[async_trait]
impl<S: UserServiceImpl> UserService for S {
    async fn find_by_code(&self, code: &str) -> Result<Option<User>> {
        UserServiceImpl::find_by_code(self, code).await
    }

    fn verify_password(&self, user: &User, password: &str) -> Result<()> {
        UserServiceImpl::verify_password(self, user, password)
    }
}
