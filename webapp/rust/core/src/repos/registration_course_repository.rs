use crate::db::{DBPool, TxConn};
use crate::models::course::Course;
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait RegistrationCourseRepository {
    async fn find_courses_by_user_id(&self, pool: &DBPool, user_id: &str) -> Result<Vec<Course>>;
    async fn find_open_courses_by_user_id_in_tx(
        &self,
        tx: &mut TxConn,
        user_id: &str,
    ) -> Result<Vec<Course>>;
    async fn find_total_scores_by_course_id_group_by_user_id(
        &self,
        pool: &DBPool,
        course_id: &str,
    ) -> Result<Vec<i64>>;
}
