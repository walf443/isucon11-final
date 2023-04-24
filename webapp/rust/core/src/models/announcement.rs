#[derive(Debug, sqlx::FromRow, PartialEq, Eq)]
pub struct Announcement {
    pub id: String,
    pub course_id: String,
    pub title: String,
    pub message: String,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct AnnouncementWithoutDetail {
    id: String,
    course_id: String,
    course_name: String,
    title: String,
    unread: bool,
}
