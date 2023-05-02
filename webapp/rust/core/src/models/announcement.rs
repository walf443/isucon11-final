use fake::{Dummy, Fake};

#[derive(Debug, sqlx::FromRow, PartialEq, Eq, Dummy)]
pub struct Announcement {
    pub id: String,
    pub course_id: String,
    pub title: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Dummy, sqlx::Type)]
#[sqlx(transparent)]
pub struct AnnouncementID(String);

impl AnnouncementID {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct AnnouncementWithoutDetail {
    pub id: String,
    pub course_id: String,
    pub course_name: String,
    pub title: String,
    pub unread: bool,
}
