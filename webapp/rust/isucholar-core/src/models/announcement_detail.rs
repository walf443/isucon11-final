#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct AnnouncementDetail {
    pub id: String,
    pub course_id: String,
    pub course_name: String,
    pub title: String,
    pub message: String,
    pub unread: bool,
}