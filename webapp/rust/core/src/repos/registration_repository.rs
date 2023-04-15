use crate::db::TxConn;
use crate::models::user::User;
use crate::repos::error::Result;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait RegistrationRepository {
    async fn create_or_update_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        course_id: &str,
    ) -> Result<()>;
    async fn exist_by_user_id_and_course_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        course_id: &str,
    ) -> Result<bool>;
    async fn find_users_by_course_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        course_id: &str,
    ) -> Result<Vec<User>>;
}

pub trait HaveRegistrationRepository {
    type Repo: RegistrationRepository + Sync;

    fn registration_repo(&self) -> &Self::Repo;
}
