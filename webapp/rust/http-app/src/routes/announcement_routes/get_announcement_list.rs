use crate::responses::error::ResponseError::InvalidPage;
use crate::responses::error::ResponseResult;
use crate::routes::util::get_user_info;
use actix_web::{web, HttpResponse};
use isucholar_core::models::announcement::AnnouncementWithoutDetail;
use isucholar_core::repos::unread_announcement_repository::{
    UnreadAnnouncementRepository, UnreadAnnouncementRepositoryImpl,
};

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
) -> ResponseResult<HttpResponse> {
    let (user_id, _, _) = get_user_info(session)?;

    let unread_announcement_repos = UnreadAnnouncementRepositoryImpl {};

    let mut tx = pool.begin().await?;

    let mut course_id: Option<&str> = None;
    if let Some(ref c_id) = params.course_id {
        course_id = Some(&c_id);
    }

    let page = if let Some(ref page_str) = params.page {
        match page_str.parse() {
            Ok(page) if page > 0 => page,
            _ => return Err(InvalidPage),
        }
    } else {
        1
    };
    let limit = 20;
    let offset = limit * (page - 1);

    let mut announcements = unread_announcement_repos
        .find_unread_announcements_by_user_id_in_tx(&mut tx, &user_id, limit, offset, course_id)
        .await?;

    let unread_count = unread_announcement_repos
        .count_unread_by_user_id_in_tx(&mut tx, &user_id)
        .await?;

    tx.commit().await?;

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
