use crate::database::DBPool;
use crate::models::course::{Course, CreateCourse};
use crate::repos::error::{ReposError, Result};
use crate::MYSQL_ERR_NUM_DUPLICATE_ENTRY;
use async_trait::async_trait;

#[async_trait]
pub trait CourseRepository {
    async fn create(&self, pool: &DBPool, course: &CreateCourse) -> Result<String>;
    async fn find_by_code(&self, pool: &DBPool, code: &str) -> Result<Course>;
}

pub struct CourseRepositoryImpl {}

#[async_trait]
impl CourseRepository for CourseRepositoryImpl {
    async fn create(&self, pool: &DBPool, req: &CreateCourse) -> Result<String> {
        let result = sqlx::query("INSERT INTO `courses` (`id`, `code`, `type`, `name`, `description`, `credit`, `period`, `day_of_week`, `teacher_id`, `keywords`) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&req.id)
            .bind(&req.code)
            .bind(&req.type_)
            .bind(&req.name)
            .bind(&req.description)
            .bind(&req.credit)
            .bind(&req.period)
            .bind(&req.day_of_week)
            .bind(&req.user_id)
            .bind(&req.keywords)
            .execute(pool)
            .await;

        if let Err(sqlx::Error::Database(ref db_error)) = result {
            if let Some(mysql_error) =
                db_error.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>()
            {
                if mysql_error.number() == MYSQL_ERR_NUM_DUPLICATE_ENTRY {
                    let course = self.find_by_code(&pool, &req.code).await?;

                    if req.type_ != course.type_
                        || req.name != course.name
                        || req.description != course.description
                        || req.credit != course.credit as i64
                        || req.period != course.period as i64
                        || req.day_of_week != course.day_of_week
                        || req.keywords != course.keywords
                    {
                        return Err(ReposError::CourseDepulicate());
                    } else {
                        return Ok(course.id);
                    }
                }
            }
        }

        result?;

        Ok(req.id.clone())
    }

    async fn find_by_code(&self, pool: &DBPool, code: &str) -> Result<Course> {
        let course: Course = sqlx::query_as("SELECT * FROM `courses` WHERE `code` = ?")
            .bind(code)
            .fetch_one(pool)
            .await?;

        Ok(course)
    }
}
