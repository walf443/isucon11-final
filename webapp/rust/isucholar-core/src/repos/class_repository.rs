use crate::database::DBPool;
use crate::models::class::Class;
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ClassRepository {
    async fn find_all_by_course_id(&self, pool: &DBPool, course_id: &str) -> Result<Vec<Class>>;
}

pub struct ClassRepositoryImpl {}

#[async_trait]
impl ClassRepository for ClassRepositoryImpl {
    async fn find_all_by_course_id(&self, pool: &DBPool, course_id: &str) -> Result<Vec<Class>> {
        let classes: Vec<Class> = sqlx::query_as(concat!(
            "SELECT *",
            " FROM `classes`",
            " WHERE `course_id` = ?",
            " ORDER BY `part` DESC",
        ))
        .bind(course_id)
        .fetch_all(pool)
        .await?;

        Ok(classes)
    }
}
