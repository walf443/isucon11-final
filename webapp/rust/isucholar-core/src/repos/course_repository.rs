use crate::database::DBPool;
use crate::models::course::Course;
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait CourseRepository {
    async fn find_by_code(&self, pool: &DBPool, code: &str) -> Result<Course>;
}

pub struct CourseRepositoryImpl {}

#[async_trait]
impl CourseRepository for CourseRepositoryImpl {
    async fn find_by_code(&self, pool: &DBPool, code: &str) -> Result<Course> {
        let course: Course = sqlx::query_as("SELECT * FROM `courses` WHERE `code` = ?")
            .bind(code)
            .fetch_one(pool)
            .await?;

        Ok(course)
    }
}
