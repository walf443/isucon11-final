use fake::{Dummy, Fake};

#[derive(Debug, sqlx::FromRow, PartialEq, Eq, Dummy)]
pub struct Announcement {
    pub id: String,
    pub course_id: String,
    pub title: String,
    pub message: String,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct AnnouncementWithoutDetail {
    pub id: String,
    pub course_id: String,
    pub course_name: String,
    pub title: String,
    pub unread: bool,
}
