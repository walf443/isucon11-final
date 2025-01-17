use crate::db::DBConn;
use crate::models::course::{Course, CourseID};
use crate::models::user::UserID;
use crate::repos::error::Result;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait RegistrationCourseRepository {
    async fn find_courses_by_user_id(
        &self,
        conn: &mut DBConn,
        user_id: &UserID,
    ) -> Result<Vec<Course>>;
    async fn find_open_courses_by_user_id(
        &self,
        conn: &mut DBConn,
        user_id: &UserID,
    ) -> Result<Vec<Course>>;
    async fn find_total_scores_by_course_id_group_by_user_id(
        &self,
        conn: &mut DBConn,
        course_id: &CourseID,
    ) -> Result<Vec<i64>>;
}

pub trait HaveRegistrationCourseRepository {
    type Repo: Sync + RegistrationCourseRepository;

    fn registration_course_repo(&self) -> &Self::Repo;
}
