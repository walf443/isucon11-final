use crate::db::{DBPool, TxConn};
use crate::models::class::{Class, ClassWithSubmitted};
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ClassRepository {
    async fn find_by_course_id_and_part(
        &self,
        pool: &DBPool,
        course_id: &str,
        part: &u8,
    ) -> Result<Class>;
    async fn find_all_by_course_id(&self, pool: &DBPool, course_id: &str) -> Result<Vec<Class>>;
    async fn find_all_with_submitteed_by_user_id_and_course_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        course_id: &str,
    ) -> Result<Vec<ClassWithSubmitted>>;
}

pub struct ClassRepositoryImpl {}

#[async_trait]
impl ClassRepository for ClassRepositoryImpl {
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

    async fn find_all_with_submitteed_by_user_id_and_course_id_in_tx<'c>(
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
