use crate::db::{DBPool, TxConn};
use crate::models::course::{Course, CourseWithTeacher, CreateCourse};
use crate::models::course_status::CourseStatus;
use crate::repos::error::{ReposError, Result};
use crate::{db, MYSQL_ERR_NUM_DUPLICATE_ENTRY};
use async_trait::async_trait;

#[async_trait]
pub trait CourseRepository {
    async fn create(&self, pool: &DBPool, course: &CreateCourse) -> Result<String>;
    async fn find_status_for_share_lock_by_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        id: &str,
    ) -> Result<Option<CourseStatus>>;
    async fn find_for_share_lock_by_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        id: &str,
    ) -> Result<Option<Course>>;
    async fn exist_by_id_in_tx<'c>(&self, tx: &mut TxConn<'c>, id: &str) -> Result<bool>;
    async fn for_update_by_id_in_tx<'c>(&self, tx: &mut TxConn<'c>, id: &str) -> Result<bool>;
    async fn update_status_by_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        id: &str,
        status: &CourseStatus,
    ) -> Result<()>;
    async fn find_by_code(&self, pool: &DBPool, code: &str) -> Result<Course>;
    async fn find_with_teacher_by_id(
        &self,
        pool: &DBPool,
        id: &str,
    ) -> Result<Option<CourseWithTeacher>>;
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
                        return Err(ReposError::CourseDepulicate);
                    } else {
                        return Ok(course.id);
                    }
                }
            }
        }

        result?;

        Ok(req.id.clone())
    }

    async fn find_status_for_share_lock_by_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        id: &str,
    ) -> Result<Option<CourseStatus>> {
        let status: Option<CourseStatus> = db::fetch_optional_scalar(
            sqlx::query_scalar("SELECT `status` FROM `courses` WHERE `id` = ? FOR SHARE").bind(id),
            tx,
        )
        .await?;

        Ok(status)
    }

    async fn find_for_share_lock_by_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        id: &str,
    ) -> Result<Option<Course>> {
        let course: Option<Course> = db::fetch_optional_as(
            sqlx::query_as("SELECT * FROM `courses` WHERE `id` = ? FOR SHARE").bind(id),
            tx,
        )
        .await?;

        Ok(course)
    }

    async fn exist_by_id_in_tx<'c>(&self, tx: &mut TxConn<'c>, id: &str) -> Result<bool> {
        let count: i64 = db::fetch_one_scalar(
            sqlx::query_scalar("SELECT COUNT(*) FROM `courses` WHERE `id` = ?").bind(id),
            tx,
        )
        .await?;

        Ok(count == 1)
    }

    async fn for_update_by_id_in_tx<'c>(&self, tx: &mut TxConn<'c>, id: &str) -> Result<bool> {
        let count: i64 = db::fetch_one_scalar(
            sqlx::query_scalar("SELECT COUNT(*) FROM `courses` WHERE `id` = ? FOR UPDATE").bind(id),
            tx,
        )
        .await?;

        Ok(count == 1)
    }

    async fn update_status_by_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        id: &str,
        status: &CourseStatus,
    ) -> Result<()> {
        sqlx::query("UPDATE `courses` SET `status` = ? WHERE `id` = ?")
            .bind(status)
            .bind(id)
            .execute(tx)
            .await?;

        Ok(())
    }

    async fn find_by_code(&self, pool: &DBPool, code: &str) -> Result<Course> {
        let course: Course = sqlx::query_as("SELECT * FROM `courses` WHERE `code` = ?")
            .bind(code)
            .fetch_one(pool)
            .await?;

        Ok(course)
    }

    async fn find_with_teacher_by_id(
        &self,
        pool: &DBPool,
        id: &str,
    ) -> Result<Option<CourseWithTeacher>> {
        let res: Option<CourseWithTeacher> = sqlx::query_as(concat!(
            "SELECT `courses`.*, `users`.`name` AS `teacher`",
            " FROM `courses`",
            " JOIN `users` ON `courses`.`teacher_id` = `users`.`id`",
            " WHERE `courses`.`id` = ?",
        ))
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(res)
    }
}
