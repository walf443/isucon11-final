use crate::db;
use crate::db::{DBPool, TxConn};
use crate::models::course_status::CourseStatus;
use crate::models::user::User;
use crate::models::user_type::UserType;
use crate::repos::error::Result;
use async_trait::async_trait;
use futures::StreamExt;
use num_traits::ToPrimitive;

#[async_trait]
pub trait UserRepository {
    async fn find_in_tx<'c>(&self, tx: &mut TxConn<'c>, id: &str) -> Result<User>;
    async fn find_by_code(&self, pool: &DBPool, code: &str) -> Result<Option<User>>;
    async fn find_code_by_id(&self, pool: &DBPool, id: &str) -> Result<String>;
    async fn find_gpas_group_by_user_id(&self, pool: &DBPool) -> Result<Vec<f64>>;
}

pub struct UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_in_tx<'c>(&self, tx: &mut TxConn<'c>, id: &str) -> Result<User> {
        let user: User = db::fetch_one_as(
            sqlx::query_as("SELECT * FROM `users` WHERE `id` = ?").bind(id),
            tx,
        )
        .await?;

        Ok(user)
    }

    async fn find_by_code(&self, pool: &DBPool, code: &str) -> Result<Option<User>> {
        let user: Option<User> = sqlx::query_as("SELECT * FROM `users` WHERE `code` = ?")
            .bind(code)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    async fn find_code_by_id(&self, pool: &DBPool, id: &str) -> Result<String> {
        let user_code = sqlx::query_scalar("SELECT `code` FROM `users` WHERE `id` = ?")
            .bind(&id)
            .fetch_one(pool)
            .await?;

        Ok(user_code)
    }

    async fn find_gpas_group_by_user_id(&self, pool: &DBPool) -> Result<Vec<f64>> {
        let gpas = {
            let mut rows = sqlx::query_scalar(concat!(
            "SELECT IFNULL(SUM(`submissions`.`score` * `courses`.`credit`), 0) / 100 / `credits`.`credits` AS `gpa`",
            " FROM `users`",
            " JOIN (",
            "     SELECT `users`.`id` AS `user_id`, SUM(`courses`.`credit`) AS `credits`",
            "     FROM `users`",
            "     JOIN `registrations` ON `users`.`id` = `registrations`.`user_id`",
            "     JOIN `courses` ON `registrations`.`course_id` = `courses`.`id` AND `courses`.`status` = ?",
            "     GROUP BY `users`.`id`",
            " ) AS `credits` ON `credits`.`user_id` = `users`.`id`",
            " JOIN `registrations` ON `users`.`id` = `registrations`.`user_id`",
            " JOIN `courses` ON `registrations`.`course_id` = `courses`.`id` AND `courses`.`status` = ?",
            " LEFT JOIN `classes` ON `courses`.`id` = `classes`.`course_id`",
            " LEFT JOIN `submissions` ON `users`.`id` = `submissions`.`user_id` AND `submissions`.`class_id` = `classes`.`id`",
            " WHERE `users`.`type` = ?",
            " GROUP BY `users`.`id`",
            ))
                .bind(CourseStatus::Closed)
                .bind(CourseStatus::Closed)
                .bind(UserType::Student)
                .fetch(pool);
            let mut gpas = Vec::new();
            while let Some(row) = rows.next().await {
                let gpa: sqlx::types::Decimal = row?;
                gpas.push(gpa.to_f64().unwrap());
            }

            gpas
        };

        Ok(gpas)
    }
}
