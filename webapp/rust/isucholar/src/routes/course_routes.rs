use crate::requests::search_courses_query::SearchCoursesQuery;
use crate::responses::error::SqlxError;
use crate::responses::get_course_detail_response::GetCourseDetailResponse;
use actix_web::{web, HttpResponse};
use sqlx::Arguments;

// GET /api/courses 科目検索
pub async fn search_courses(
    pool: web::Data<sqlx::MySqlPool>,
    params: web::Query<SearchCoursesQuery>,
    request: actix_web::HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let query = concat!(
        "SELECT `courses`.*, `users`.`name` AS `teacher`",
        " FROM `courses` JOIN `users` ON `courses`.`teacher_id` = `users`.`id`",
        " WHERE 1=1",
    );
    let mut condition = String::new();
    let mut args = sqlx::mysql::MySqlArguments::default();

    // 無効な検索条件はエラーを返さず無視して良い

    if let Some(ref course_type) = params.type_ {
        condition.push_str(" AND `courses`.`type` = ?");
        args.add(course_type);
    }

    if let Some(credit) = params.credit {
        if credit > 0 {
            condition.push_str(" AND `courses`.`credit` = ?");
            args.add(credit);
        }
    }

    if let Some(ref teacher) = params.teacher {
        condition.push_str(" AND `users`.`name` = ?");
        args.add(teacher);
    }

    if let Some(period) = params.period {
        if period > 0 {
            condition.push_str(" AND `courses`.`period` = ?");
            args.add(period);
        }
    }

    if let Some(ref day_of_week) = params.day_of_week {
        condition.push_str(" AND `courses`.`day_of_week` = ?");
        args.add(day_of_week);
    }

    if let Some(ref keywords) = params.keywords {
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

    if let Some(ref status) = params.status {
        condition.push_str(" AND `courses`.`status` = ?");
        args.add(status);
    }

    condition.push_str(" ORDER BY `courses`.`code`");

    let page = if let Some(ref page_str) = params.page {
        match page_str.parse() {
            Ok(page) if page > 0 => page,
            _ => return Err(actix_web::error::ErrorBadRequest("Invalid page.")),
        }
    } else {
        1
    };
    let limit = 20;
    let offset = limit * (page - 1);

    // limitより多く上限を設定し、実際にlimitより多くレコードが取得できた場合は次のページが存在する
    condition.push_str(" LIMIT ? OFFSET ?");
    args.add(limit + 1);
    args.add(offset);

    // 結果が0件の時は空配列を返却
    let mut res: Vec<GetCourseDetailResponse> =
        sqlx::query_as_with(&format!("{}{}", query, condition), args)
            .fetch_all(pool.as_ref())
            .await
            .map_err(SqlxError)?;

    let uri = request.uri();
    let mut params = params.into_inner();
    let mut links = Vec::new();
    if page > 1 {
        params.page = Some(format!("{}", page - 1));
        links.push(format!(
            "<{}?{}>; rel=\"prev\"",
            uri.path(),
            serde_urlencoded::to_string(&params)?
        ));
    }
    if res.len() as i64 > limit {
        params.page = Some(format!("{}", page + 1));
        links.push(format!(
            "<{}?{}>; rel=\"next\"",
            uri.path(),
            serde_urlencoded::to_string(&params)?
        ));
    }

    if res.len() as i64 == limit + 1 {
        res.truncate(res.len() - 1);
    }

    let mut builder = HttpResponse::Ok();
    if !links.is_empty() {
        builder.insert_header((actix_web::http::header::LINK, links.join(",")));
    }
    Ok(builder.json(res))
}
