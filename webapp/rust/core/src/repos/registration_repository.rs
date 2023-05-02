use crate::db::DBConn;
use crate::models::course::CourseID;
use crate::models::user::{User, UserID};
use crate::repos::error::Result;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait RegistrationRepository {
    async fn create_or_update(
        &self,
        conn: &mut DBConn,
        user_id: &UserID,
        course_id: &CourseID,
    ) -> Result<()>;
    async fn exist_by_user_id_and_course_id(
        &self,
        conn: &mut DBConn,
        user_id: &UserID,
        course_id: &CourseID,
    ) -> Result<bool>;
    async fn find_users_by_course_id(
        &self,
        conn: &mut DBConn,
        course_id: &str,
    ) -> Result<Vec<User>>;
}

pub trait HaveRegistrationRepository {
    type Repo: RegistrationRepository + Sync;

    fn registration_repo(&self) -> &Self::Repo;
}
