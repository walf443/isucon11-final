use async_trait::async_trait;
use isucholar_core::db::DBConn;
use isucholar_core::models::class::{Class, ClassWithSubmitted, CreateClass};
use isucholar_core::models::user::UserID;
use isucholar_core::repos::class_repository::ClassRepository;
use isucholar_core::repos::error::ReposError::ClassDuplicate;
use isucholar_core::repos::error::Result;
use isucholar_core::MYSQL_ERR_NUM_DUPLICATE_ENTRY;

#[cfg(test)]
mod create;
#[cfg(test)]
mod find_all_by_course_id;
#[cfg(test)]
mod find_all_with_submitted_by_user_id_and_course_id;
#[cfg(test)]
mod find_by_course_id_and_part;
#[cfg(test)]
mod find_submission_closed_by_id_with_shared_lock;
#[cfg(test)]
mod for_update_by_id;
#[cfg(test)]
mod update_submission_closed_by_id;

#[derive(Clone)]
pub struct ClassRepositoryInfra {}

#[async_trait]
impl ClassRepository for ClassRepositoryInfra {
    async fn for_update_by_id(&self, conn: &mut DBConn, id: &str) -> Result<bool> {
        let class_count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM `classes` WHERE `id` = ? FOR UPDATE",
            id
        )
        .fetch_one(conn)
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

    async fn update_submission_closed_by_id(&self, conn: &mut DBConn, id: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE `classes` SET `submission_closed` = true WHERE `id` = ?",
            id
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    async fn find_submission_closed_by_id_with_shared_lock(
        &self,
        conn: &mut DBConn,
        id: &str,
    ) -> Result<Option<bool>> {
        let submission_closed = sqlx::query_scalar!(
            "SELECT `submission_closed` AS `s:bool` FROM `classes` WHERE `id` = ? FOR SHARE",
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(submission_closed)
    }

    async fn find_by_course_id_and_part(
        &self,
        conn: &mut DBConn,
        course_id: &str,
        part: &u8,
    ) -> Result<Class> {
        let class = sqlx::query_as!(
            Class,
            r"
                SELECT
                  id,
                  course_id,
                  part,
                  title,
                  description,
                  submission_closed AS `submission_closed:bool`
                FROM `classes`
                WHERE `course_id` = ? AND `part` = ?
            ",
            course_id,
            part
        )
        .fetch_one(conn)
        .await?;

        Ok(class)
    }

    async fn find_all_by_course_id(
        &self,
        conn: &mut DBConn,
        course_id: &str,
    ) -> Result<Vec<Class>> {
        let classes: Vec<Class> = sqlx::query_as!(
            Class,
            r"
            SELECT
              id,
              course_id,
              part,
              title,
              description,
              submission_closed AS `submission_closed:bool`
            FROM `classes`
            WHERE `course_id` = ?
            ORDER BY `part` DESC
        ",
            course_id
        )
        .fetch_all(conn)
        .await?;

        Ok(classes)
    }

    async fn find_all_with_submitted_by_user_id_and_course_id(
        &self,
        conn: &mut DBConn,
        user_id: &UserID,
        course_id: &str,
    ) -> Result<Vec<ClassWithSubmitted>> {
        let classes: Vec<ClassWithSubmitted> = sqlx::query_as!(ClassWithSubmitted,
            r"
                SELECT
                  `classes`.id,
                  `classes`.course_id,
                  `classes`.part,
                  `classes`.title,
                  `classes`.description,
                  `classes`.submission_closed AS `submission_closed:bool`,
                  `submissions`.`user_id` IS NOT NULL AS `submitted:bool`
                FROM `classes`
                LEFT JOIN `submissions` ON `classes`.`id` = `submissions`.`class_id` AND `submissions`.`user_id` = ?
                WHERE `classes`.`course_id` = ?
                ORDER BY `classes`.`part`
            ",
            user_id.to_string(),
            course_id,
        )
            .fetch_all(conn)
            .await?;

        Ok(classes)
    }
}
