use async_trait::async_trait;
use isucholar_core::db::{DBConn, DBPool};
use isucholar_core::models::course::{
    Course, CourseCode, CourseID, CourseWithTeacher, CreateCourse,
};
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use isucholar_core::models::user::UserID;
use isucholar_core::repos::course_repository::{CourseRepository, SearchCoursesQuery};
use isucholar_core::repos::error::{ReposError, Result};
use isucholar_core::MYSQL_ERR_NUM_DUPLICATE_ENTRY;
use sqlx::Arguments;

#[cfg(test)]
mod create;
#[cfg(test)]
mod exist_by_id;
#[cfg(test)]
mod find_by_code;
#[cfg(test)]
mod find_for_share_lock_by_id;
#[cfg(test)]
mod find_status_for_share_lock_by_id;
#[cfg(test)]
mod find_with_teacher_by_id;
#[cfg(test)]
mod for_update_by_id;
#[cfg(test)]
mod update_status_by_id;

#[derive(Clone)]
pub struct CourseRepositoryInfra {}

#[async_trait]
impl CourseRepository for CourseRepositoryInfra {
    async fn create(&self, conn: &mut DBConn, req: &CreateCourse) -> Result<CourseID> {
        let result = sqlx::query!(
            "INSERT INTO `courses` (`id`, `code`, `type`, `name`, `description`, `credit`, `period`, `day_of_week`, `teacher_id`, `keywords`) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            &req.id,
            &req.code,
            &req.type_,
            &req.name,
            &req.description,
            &req.credit,
            &req.period,
            &req.day_of_week,
            &req.teacher_id,
            &req.keywords,
        )
            .execute(conn)
            .await;

        if let Err(sqlx::Error::Database(ref db_error)) = result {
            if let Some(mysql_error) =
                db_error.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>()
            {
                if mysql_error.number() == MYSQL_ERR_NUM_DUPLICATE_ENTRY {
                    return Err(ReposError::CourseDuplicate);
                }
            }
        }

        result?;

        Ok(req.id.clone())
    }

    async fn find_all_with_teacher(
        &self,
        pool: &DBPool,
        limit: i64,
        offset: i64,
        q: &SearchCoursesQuery,
    ) -> Result<Vec<CourseWithTeacher>> {
        let query = concat!(
            "SELECT `courses`.*, `users`.`name` AS `teacher`",
            " FROM `courses` JOIN `users` ON `courses`.`teacher_id` = `users`.`id`",
            " WHERE 1=1",
        );
        let mut condition = String::new();
        let mut args = sqlx::mysql::MySqlArguments::default();

        // 無効な検索条件はエラーを返さず無視して良い

        if let Some(ref course_type) = q.type_ {
            condition.push_str(" AND `courses`.`type` = ?");
            args.add(course_type);
        }

        if let Some(credit) = q.credit {
            if credit > 0 {
                condition.push_str(" AND `courses`.`credit` = ?");
                args.add(credit);
            }
        }

        if let Some(ref teacher) = q.teacher {
            condition.push_str(" AND `users`.`name` = ?");
            args.add(teacher);
        }

        if let Some(period) = q.period {
            if period > 0 {
                condition.push_str(" AND `courses`.`period` = ?");
                args.add(period);
            }
        }

        if let Some(ref day_of_week) = q.day_of_week {
            condition.push_str(" AND `courses`.`day_of_week` = ?");
            args.add(day_of_week);
        }

        if let Some(ref keywords) = q.keywords {
            let arr = keywords.split(' ').collect::<Vec<_>>();
            let mut name_condition = String::new();
            for keyword in &arr {
                name_condition.push_str(" AND `courses`.`name` LIKE ?");
                args.add(format!("%{}%", keyword));
            }
            let mut keywords_condition = String::new();
            for keyword in arr {
                keywords_condition.push_str(" AND `courses`.`keywords` LIKE ?");
                args.add(format!("%{}%", keyword));
            }
            condition.push_str(&format!(
                " AND ((1=1{}) OR (1=1{}))",
                name_condition, keywords_condition
            ));
        }

        if let Some(ref status) = q.status {
            condition.push_str(" AND `courses`.`status` = ?");
            args.add(status);
        }

        condition.push_str(" ORDER BY `courses`.`code`");

        // limitより多く上限を設定し、実際にlimitより多くレコードが取得できた場合は次のページが存在する
        condition.push_str(" LIMIT ? OFFSET ?");
        args.add(limit + 1);
        args.add(offset);

        // 結果が0件の時は空配列を返却
        let courses: Vec<CourseWithTeacher> =
            sqlx::query_as_with(&format!("{}{}", query, condition), args)
                .fetch_all(pool)
                .await?;

        Ok(courses)
    }

    async fn find_status_for_share_lock_by_id(
        &self,
        conn: &mut DBConn,
        id: &CourseID,
    ) -> Result<Option<CourseStatus>> {
        let status = sqlx::query_scalar!(
            "SELECT `status` AS `status:CourseStatus` FROM `courses` WHERE `id` = ? FOR SHARE",
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(status)
    }

    async fn find_for_share_lock_by_id(
        &self,
        conn: &mut DBConn,
        id: &CourseID,
    ) -> Result<Option<Course>> {
        let course: Option<Course> =
            sqlx::query_as("SELECT * FROM `courses` WHERE `id` = ? FOR SHARE")
                .bind(id)
                .fetch_optional(conn)
                .await?;

        Ok(course)
    }

    async fn exist_by_id(&self, conn: &mut DBConn, id: &CourseID) -> Result<bool> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM `courses` WHERE `id` = ?", id)
            .fetch_one(conn)
            .await?;

        Ok(count == 1)
    }

    async fn for_update_by_id(&self, conn: &mut DBConn, id: &CourseID) -> Result<bool> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM `courses` WHERE `id` = ? FOR UPDATE",
            id
        )
        .fetch_one(conn)
        .await?;

        Ok(count == 1)
    }

    async fn update_status_by_id(
        &self,
        conn: &mut DBConn,
        id: &CourseID,
        status: &CourseStatus,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE `courses` SET `status` = ? WHERE `id` = ?",
            status,
            id
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    async fn find_by_code(&self, conn: &mut DBConn, code: &CourseCode) -> Result<Course> {
        let course = sqlx::query_as!(
            Course,
            r"
                SELECT
                   id as `id:CourseID`,
                   code as `code:CourseCode`,
                   type as `type_:CourseType`,
                   name,
                   description,
                   credit,
                   period,
                   day_of_week as `day_of_week:DayOfWeek`,
                   teacher_id as `teacher_id:UserID`,
                   keywords,
                   status as `status:CourseStatus`
                FROM `courses`
                WHERE `code` = ?",
            code
        )
        .fetch_one(conn)
        .await?;

        Ok(course)
    }

    async fn find_with_teacher_by_id(
        &self,
        conn: &mut DBConn,
        id: &CourseID,
    ) -> Result<Option<CourseWithTeacher>> {
        let res: Option<CourseWithTeacher> = sqlx::query_as!(
            CourseWithTeacher,
            r"
                SELECT
                   courses.id as `id:CourseID`,
                   courses.code as `code:CourseCode`,
                   courses.type as `type_`,
                   courses.name,
                   description,
                   credit,
                   period,
                   day_of_week as `day_of_week:DayOfWeek`,
                   teacher_id as `teacher_id:UserID`,
                   keywords,
                   status as `status:CourseStatus`,
                    `users`.`name` AS `teacher`
                FROM `courses`
                JOIN `users` ON `courses`.`teacher_id` = `users`.`id`
                WHERE `courses`.`id` = ?
            ",
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(res)
    }
}
