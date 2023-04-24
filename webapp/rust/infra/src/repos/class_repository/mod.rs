use crate::db;
use async_trait::async_trait;
use isucholar_core::db::{DBConn, DBPool, TxConn};
use isucholar_core::models::class::{Class, ClassWithSubmitted, CreateClass};
use isucholar_core::repos::class_repository::ClassRepository;
use isucholar_core::repos::error::ReposError::ClassDuplicate;
use isucholar_core::repos::error::Result;
use isucholar_core::MYSQL_ERR_NUM_DUPLICATE_ENTRY;

#[cfg(test)]
mod create;
#[cfg(test)]
mod for_update_by_id;
#[cfg(test)]
mod update_submission_closed_by_id;

#[derive(Clone)]
pub struct ClassRepositoryInfra {}

#[async_trait]
impl ClassRepository for ClassRepositoryInfra {
    async fn for_update_by_id<'c>(&self, tx: &mut TxConn<'c>, id: &str) -> Result<bool> {
        let class_count: i64 = db::fetch_one_scalar(
            sqlx::query_scalar!(
                "SELECT COUNT(*) FROM `classes` WHERE `id` = ? FOR UPDATE",
                id
            ),
            tx,
        )
        .await?;

        Ok(class_count == 1)
    }

    async fn create(&self, conn: &mut DBConn, class: &CreateClass) -> Result<()> {
        let result = sqlx::query!("INSERT INTO `classes` (`id`, `course_id`, `part`, `title`, `description`) VALUES (?, ?, ?, ?, ?)",
            &class.id,
            &class.course_id,
            &class.part,
            &class.title,
            &class.description
        )
            .execute(conn)
            .await;

        if let Err(e) = result {
            if let sqlx::error::Error::Database(ref db_error) = e {
                if let Some(mysql_error) =
                    db_error.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>()
                {
                    if mysql_error.number() == MYSQL_ERR_NUM_DUPLICATE_ENTRY {
                        return Err(ClassDuplicate);
                    }
                }
            }

            return Err(e.into());
        }

        Ok(())
    }

    async fn update_submission_closed_by_id<'c>(
        &self,
        tx: &mut TxConn<'c>,
        id: &str,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE `classes` SET `submission_closed` = true WHERE `id` = ?",
            id
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    async fn find_submission_closed_by_id<'c>(
        &self,
        tx: &mut TxConn<'c>,
        id: &str,
    ) -> Result<Option<bool>> {
        let submission_closed: Option<bool> = db::fetch_optional_scalar(
            sqlx::query_scalar(
                "SELECT `submission_closed` FROM `classes` WHERE `id` = ? FOR SHARE",
            )
            .bind(id),
            tx,
        )
        .await?;

        Ok(submission_closed)
    }

    async fn find_by_course_id_and_part(
        &self,
        pool: &DBPool,
        course_id: &str,
        part: &u8,
    ) -> Result<Class> {
        let class: Class =
            sqlx::query_as("SELECT * FROM `classes` WHERE `course_id` = ? AND `part` = ?")
                .bind(course_id)
                .bind(part)
                .fetch_one(pool)
                .await?;

        Ok(class)
    }

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

    async fn find_all_with_submitted_by_user_id_and_course_id<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        course_id: &str,
    ) -> Result<Vec<ClassWithSubmitted>> {
        let classes: Vec<ClassWithSubmitted> = sqlx::query_as(concat!(
        "SELECT `classes`.*, `submissions`.`user_id` IS NOT NULL AS `submitted`",
        " FROM `classes`",
        " LEFT JOIN `submissions` ON `classes`.`id` = `submissions`.`class_id` AND `submissions`.`user_id` = ?",
        " WHERE `classes`.`course_id` = ?",
        " ORDER BY `classes`.`part`",
        ))
            .bind(user_id)
            .bind(course_id)
            .fetch_all(tx)
            .await?;

        Ok(classes)
    }
}
