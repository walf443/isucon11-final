use crate::responses::error::SqlxError;
use crate::responses::get_grade_response::GetGradeResponse;
use crate::routes::util::get_user_info;
use crate::util;
use actix_web::{web, HttpResponse};
use futures::StreamExt;
use isucholar_core::models::class::Class;
use isucholar_core::models::class_score::ClassScore;
use isucholar_core::models::course::Course;
use isucholar_core::models::course_result::CourseResult;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::summary::Summary;
use isucholar_core::models::user_type::UserType;
use num_traits::ToPrimitive;

// GET /api/users/me/grades 成績取得
pub async fn get_grades(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    // 履修している科目一覧取得
    let registered_courses: Vec<Course> = sqlx::query_as(concat!(
        "SELECT `courses`.*",
        " FROM `registrations`",
        " JOIN `courses` ON `registrations`.`course_id` = `courses`.`id`",
        " WHERE `user_id` = ?",
    ))
    .bind(&user_id)
    .fetch_all(pool.as_ref())
    .await
    .map_err(SqlxError)?;

    // 科目毎の成績計算処理
    let mut course_results = Vec::with_capacity(registered_courses.len());
    let mut my_gpa = 0f64;
    let mut my_credits = 0;
    for course in registered_courses {
        // 講義一覧の取得
        let classes: Vec<Class> = sqlx::query_as(concat!(
            "SELECT *",
            " FROM `classes`",
            " WHERE `course_id` = ?",
            " ORDER BY `part` DESC",
        ))
        .bind(&course.id)
        .fetch_all(pool.as_ref())
        .await
        .map_err(SqlxError)?;

        // 講義毎の成績計算処理
        let mut class_scores = Vec::with_capacity(classes.len());
        let mut my_total_score = 0;
        for class in classes {
            let submissions_count: i64 =
                sqlx::query_scalar("SELECT COUNT(*) FROM `submissions` WHERE `class_id` = ?")
                    .bind(&class.id)
                    .fetch_one(pool.as_ref())
                    .await
                    .map_err(SqlxError)?;

            let my_score: Option<Option<u8>> = sqlx::query_scalar(concat!(
                "SELECT `submissions`.`score` FROM `submissions`",
                " WHERE `user_id` = ? AND `class_id` = ?"
            ))
            .bind(&user_id)
            .bind(&class.id)
            .fetch_optional(pool.as_ref())
            .await
            .map_err(SqlxError)?;
            if let Some(Some(my_score)) = my_score {
                let my_score = my_score as i64;
                my_total_score += my_score;
                class_scores.push(ClassScore {
                    class_id: class.id,
                    part: class.part,
                    title: class.title,
                    score: Some(my_score),
                    submitters: submissions_count,
                });
            } else {
                class_scores.push(ClassScore {
                    class_id: class.id,
                    part: class.part,
                    title: class.title,
                    score: None,
                    submitters: submissions_count,
                });
            }
        }

        // この科目を履修している学生のtotal_score一覧を取得
        let mut rows = sqlx::query_scalar(concat!(
        "SELECT IFNULL(SUM(`submissions`.`score`), 0) AS `total_score`",
        " FROM `users`",
        " JOIN `registrations` ON `users`.`id` = `registrations`.`user_id`",
        " JOIN `courses` ON `registrations`.`course_id` = `courses`.`id`",
        " LEFT JOIN `classes` ON `courses`.`id` = `classes`.`course_id`",
        " LEFT JOIN `submissions` ON `users`.`id` = `submissions`.`user_id` AND `submissions`.`class_id` = `classes`.`id`",
        " WHERE `courses`.`id` = ?",
        " GROUP BY `users`.`id`",
        ))
            .bind(&course.id)
            .fetch(pool.as_ref());
        let mut totals = Vec::new();
        while let Some(row) = rows.next().await {
            let total_score: sqlx::types::Decimal = row.map_err(SqlxError)?;
            totals.push(total_score.to_i64().unwrap());
        }

        course_results.push(CourseResult {
            name: course.name,
            code: course.code,
            total_score: my_total_score,
            total_score_t_score: util::t_score_int(my_total_score, &totals),
            total_score_avg: util::average_int(&totals, 0.0),
            total_score_max: util::max_int(&totals, 0),
            total_score_min: util::min_int(&totals, 0),
            class_scores,
        });

        // 自分のGPA計算
        if course.status == CourseStatus::Closed {
            my_gpa += (my_total_score * course.credit as i64) as f64;
            my_credits += course.credit as i64;
        }
    }
    if my_credits > 0 {
        my_gpa = my_gpa / 100.0 / my_credits as f64;
    }

    // GPAの統計値
    // 一つでも修了した科目がある学生のGPA一覧
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
            .fetch(pool.as_ref());
        let mut gpas = Vec::new();
        while let Some(row) = rows.next().await {
            let gpa: sqlx::types::Decimal = row.map_err(SqlxError)?;
            gpas.push(gpa.to_f64().unwrap());
        }
        gpas
    };

    Ok(HttpResponse::Ok().json(GetGradeResponse {
        course_results,
        summary: Summary {
            credits: my_credits,
            gpa: my_gpa,
            gpa_t_score: util::t_score_f64(my_gpa, &gpas),
            gpa_avg: util::average_f64(&gpas, 0.0),
            gpa_max: util::max_f64(&gpas, 0.0),
            gpa_min: util::min_f64(&gpas, 0.0),
        },
    }))
}
