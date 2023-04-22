use crate::db;
use async_trait::async_trait;
use futures::StreamExt;
use isucholar_core::db::{DBPool, TxConn};
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::user::User;
use isucholar_core::models::user_type::UserType;
use isucholar_core::repos::error::Result;
use isucholar_core::repos::user_repository::UserRepository;
use num_traits::ToPrimitive;

#[derive(Clone)]
pub struct UserRepositoryInfra {}

#[async_trait]
impl UserRepository for UserRepositoryInfra {
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

    async fn find_code_by_id(&self, pool: &DBPool, id: &str) -> Result<Option<String>> {
        let user_code: Option<String> =
            sqlx::query_scalar("SELECT `code` FROM `users` WHERE `id` = ?")
                .bind(&id)
                .fetch_optional(pool)
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
