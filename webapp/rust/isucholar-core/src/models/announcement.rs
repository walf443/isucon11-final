
#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct AnnouncementWithoutDetail {
    id: String,
    course_id: String,
    course_name: String,
    title: String,
    unread: bool,
}

