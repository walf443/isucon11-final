use async_trait::async_trait;
use isucholar_core::db::{DBPool, TxConn};
use isucholar_core::models::course::{Course, CourseWithTeacher, CreateCourse};
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::repos::course_repository::{CourseRepository, SearchCoursesQuery};
use isucholar_core::repos::error::{ReposError, Result};
use isucholar_core::{db, MYSQL_ERR_NUM_DUPLICATE_ENTRY};
use sqlx::Arguments;

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
                        return Err(ReposError::CourseDuplicate);
                    } else {
                        return Ok(course.id);
                    }
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
