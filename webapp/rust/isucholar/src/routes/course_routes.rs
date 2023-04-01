use crate::requests::search_courses_query::SearchCoursesQuery;
use crate::responses::error::SqlxError;
use crate::responses::get_course_detail_response::GetCourseDetailResponse;
use crate::routes::util::get_user_info;
use crate::{db, util};
use actix_web::{web, HttpResponse};
use isucholar_core::models::class::ClassWithSubmitted;
use isucholar_core::models::course::Course;
use isucholar_core::models::course_status::CourseStatus;
use isucholar_core::models::course_type::CourseType;
use isucholar_core::models::day_of_week::DayOfWeek;
use sqlx::Arguments;

const MYSQL_ERR_NUM_DUPLICATE_ENTRY: u16 = 1062;

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

#[derive(Debug, serde::Deserialize)]
pub struct AddCourseRequest {
    code: String,
    #[serde(rename = "type")]
    type_: CourseType,
    name: String,
    description: String,
    credit: i64,
    period: i64,
    day_of_week: DayOfWeek,
    keywords: String,
}

#[derive(Debug, serde::Serialize)]
pub struct AddCourseResponse {
    pub id: String,
}

// POST /api/courses 新規科目登録
pub async fn add_course(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    req: web::Json<AddCourseRequest>,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_id = util::new_ulid().await;
    let result = sqlx::query("INSERT INTO `courses` (`id`, `code`, `type`, `name`, `description`, `credit`, `period`, `day_of_week`, `teacher_id`, `keywords`) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&course_id)
        .bind(&req.code)
        .bind(&req.type_)
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.credit)
        .bind(&req.period)
        .bind(&req.day_of_week)
        .bind(&user_id)
        .bind(&req.keywords)
        .execute(pool.as_ref())
        .await;
    if let Err(sqlx::Error::Database(ref db_error)) = result {
        if let Some(mysql_error) = db_error.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>() {
            if mysql_error.number() == MYSQL_ERR_NUM_DUPLICATE_ENTRY {
                let course: Course = sqlx::query_as("SELECT * FROM `courses` WHERE `code` = ?")
                    .bind(&req.code)
                    .fetch_one(pool.as_ref())
                    .await
                    .map_err(SqlxError)?;
                if req.type_ != course.type_
                    || req.name != course.name
                    || req.description != course.description
                    || req.credit != course.credit as i64
                    || req.period != course.period as i64
                    || req.day_of_week != course.day_of_week
                    || req.keywords != course.keywords
                {
                    return Err(actix_web::error::ErrorConflict(
                        "A course with the same code already exists.",
                    ));
                } else {
                    return Ok(HttpResponse::Created().json(AddCourseResponse { id: course.id }));
                }
            }
        }
    }
    result.map_err(SqlxError)?;

    Ok(HttpResponse::Created().json(AddCourseResponse { id: course_id }))
}

// GET /api/courses/{course_id} 科目詳細の取得
pub async fn get_course_detail(
    pool: web::Data<sqlx::MySqlPool>,
    course_id: web::Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let course_id = &course_id.0;

    let res: Option<GetCourseDetailResponse> = sqlx::query_as(concat!(
        "SELECT `courses`.*, `users`.`name` AS `teacher`",
        " FROM `courses`",
        " JOIN `users` ON `courses`.`teacher_id` = `users`.`id`",
        " WHERE `courses`.`id` = ?",
    ))
    .bind(course_id)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(SqlxError)?;

    if let Some(res) = res {
        Ok(HttpResponse::Ok().json(res))
    } else {
        Err(actix_web::error::ErrorNotFound("No such course."))
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct SetCourseStatusRequest {
    status: CourseStatus,
}

// PUT /api/courses/{course_id}/status 科目のステータスを変更
pub async fn set_course_status(
    pool: web::Data<sqlx::MySqlPool>,
    course_id: web::Path<(String,)>,
    req: web::Json<SetCourseStatusRequest>,
) -> actix_web::Result<HttpResponse> {
    let course_id = &course_id.0;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let count: i64 = db::fetch_one_scalar(
        sqlx::query_scalar("SELECT COUNT(*) FROM `courses` WHERE `id` = ? FOR UPDATE")
            .bind(course_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if count == 0 {
        return Err(actix_web::error::ErrorNotFound("No such course."));
    }

    sqlx::query("UPDATE `courses` SET `status` = ? WHERE `id` = ?")
        .bind(&req.status)
        .bind(course_id)
        .execute(&mut tx)
        .await
        .map_err(SqlxError)?;

    tx.commit().await.map_err(SqlxError)?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug, serde::Serialize)]
struct GetClassResponse {
    id: String,
    part: u8,
    title: String,
    description: String,
    submission_closed: bool,
    submitted: bool,
}

// GET /api/courses/{course_id}/classes 科目に紐づく講義一覧の取得
pub async fn get_classes(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    course_id: web::Path<(String,)>,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let course_id = &course_id.0;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let count: i64 = db::fetch_one_scalar(
        sqlx::query_scalar("SELECT COUNT(*) FROM `courses` WHERE `id` = ?").bind(course_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;
    if count == 0 {
        return Err(actix_web::error::ErrorNotFound("No such course."));
    }

    let classes: Vec<ClassWithSubmitted> = sqlx::query_as(concat!(
    "SELECT `classes`.*, `submissions`.`user_id` IS NOT NULL AS `submitted`",
    " FROM `classes`",
    " LEFT JOIN `submissions` ON `classes`.`id` = `submissions`.`class_id` AND `submissions`.`user_id` = ?",
    " WHERE `classes`.`course_id` = ?",
    " ORDER BY `classes`.`part`",
    ))
        .bind(&user_id)
        .bind(course_id)
        .fetch_all(&mut tx)
        .await
        .map_err(SqlxError)?;

    tx.commit().await.map_err(SqlxError)?;

    // 結果が0件の時は空配列を返却
    let res = classes
        .into_iter()
        .map(|class| GetClassResponse {
            id: class.id,
            part: class.part,
            title: class.title,
            description: class.description,
            submission_closed: class.submission_closed,
            submitted: class.submitted,
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(res))
}
