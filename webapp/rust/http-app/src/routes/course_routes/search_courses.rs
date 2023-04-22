use actix_web::{web, HttpResponse};
use isucholar_core::repos::course_repository::{CourseRepository, SearchCoursesQuery};
use isucholar_http_core::responses::error::ResponseError::InvalidPage;
use isucholar_http_core::responses::error::ResponseResult;
use isucholar_infra::repos::course_repository::CourseRepositoryInfra;

// GET /api/courses 科目検索
pub async fn search_courses(
    pool: web::Data<sqlx::MySqlPool>,
    params: web::Query<SearchCoursesQuery>,
    request: actix_web::HttpRequest,
) -> ResponseResult<HttpResponse> {
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

    let course_repo = CourseRepositoryInfra {};
    let mut res = course_repo
        .find_all_with_teacher(&pool, limit, offset, &params)
        .await?;

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
