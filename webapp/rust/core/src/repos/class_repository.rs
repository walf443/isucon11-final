use crate::db::DBConn;
use crate::models::class::{Class, ClassWithSubmitted, CreateClass};
use crate::models::user::UserID;
use crate::repos::error::Result;
use async_trait::async_trait;
use crate::models::course::{CourseID};

#[async_trait]
pub trait ClassRepository {
    async fn for_update_by_id(&self, conn: &mut DBConn, id: &str) -> Result<bool>;
    async fn create(&self, conn: &mut DBConn, class: &CreateClass) -> Result<()>;
    async fn update_submission_closed_by_id(&self, conn: &mut DBConn, id: &str) -> Result<()>;
    async fn find_submission_closed_by_id_with_shared_lock(
        &self,
        conn: &mut DBConn,
        id: &str,
    ) -> Result<Option<bool>>;
    async fn find_by_course_id_and_part(
        &self,
        conn: &mut DBConn,
        course_id: &CourseID,
        part: &u8,
    ) -> Result<Class>;
    async fn find_all_by_course_id(&self, conn: &mut DBConn, course_id: &CourseID)
        -> Result<Vec<Class>>;
    async fn find_all_with_submitted_by_user_id_and_course_id(
        &self,
        conn: &mut DBConn,
        user_id: &UserID,
        course_id: &CourseID,
    ) -> Result<Vec<ClassWithSubmitted>>;
}

pub trait HaveClassRepository {
    type Repo: Sync + ClassRepository;

    fn class_repo(&self) -> &Self::Repo;
}
