use crate::db;
use crate::responses::error::SqlxError;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::models::announcement::AnnouncementWithoutDetail;
use sqlx::Arguments;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GetAnnouncementsQuery {
    course_id: Option<String>,
    page: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct GetAnnouncementsResponse {
    unread_count: i64,
    announcements: Vec<AnnouncementWithoutDetail>,
}

// GET /api/announcements お知らせ一覧取得
pub async fn get_announcement_list(
    pool: web::Data<sqlx::MySqlPool>,
    session: actix_session::Session,
    params: web::Query<GetAnnouncementsQuery>,
    request: actix_web::HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let mut tx = pool.begin().await.map_err(SqlxError)?;

    let mut query = concat!(
    "SELECT `announcements`.`id`, `courses`.`id` AS `course_id`, `courses`.`name` AS `course_name`, `announcements`.`title`, NOT `unread_announcements`.`is_deleted` AS `unread`",
    " FROM `announcements`",
    " JOIN `courses` ON `announcements`.`course_id` = `courses`.`id`",
    " JOIN `registrations` ON `courses`.`id` = `registrations`.`course_id`",
    " JOIN `unread_announcements` ON `announcements`.`id` = `unread_announcements`.`announcement_id`",
    " WHERE 1=1",
    ).to_owned();
    let mut args = sqlx::mysql::MySqlArguments::default();

    if let Some(ref course_id) = params.course_id {
        query.push_str(" AND `announcements`.`course_id` = ?");
        args.add(course_id);
    }

    query.push_str(concat!(
        " AND `unread_announcements`.`user_id` = ?",
        " AND `registrations`.`user_id` = ?",
        " ORDER BY `announcements`.`id` DESC",
        " LIMIT ? OFFSET ?",
    ));
    args.add(&user_id);
    args.add(&user_id);

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
    args.add(limit + 1);
    args.add(offset);

    let mut announcements: Vec<AnnouncementWithoutDetail> = sqlx::query_as_with(&query, args)
        .fetch_all(&mut tx)
        .await
        .map_err(SqlxError)?;

    let unread_count: i64 = db::fetch_one_scalar(
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM `unread_announcements` WHERE `user_id` = ? AND NOT `is_deleted`",
        )
        .bind(&user_id),
        &mut tx,
    )
    .await
    .map_err(SqlxError)?;

    tx.commit().await.map_err(SqlxError)?;

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
    if announcements.len() as i64 > limit {
        params.page = Some(format!("{}", page + 1));
        links.push(format!(
            "<{}?{}>; rel=\"next\"",
            uri.path(),
            serde_urlencoded::to_string(&params)?
        ));
    }

    if announcements.len() as i64 == limit + 1 {
        announcements.truncate(announcements.len() - 1);
    }

    // 対象になっているお知らせが0件の時は空配列を返却

    let mut builder = HttpResponse::Ok();
    if !links.is_empty() {
        builder.insert_header((actix_web::http::header::LINK, links.join(",")));
    }
    Ok(builder.json(GetAnnouncementsResponse {
        unread_count,
        announcements,
    }))
}
