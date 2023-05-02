use crate::models::course::CourseID;
use fake::{Dummy, Fake};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, PartialEq, Eq, Dummy)]
pub struct Announcement {
    pub id: AnnouncementID,
    pub course_id: CourseID,
    pub title: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Dummy, sqlx::Type, Serialize, Deserialize)]
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
    pub id: AnnouncementID,
    pub course_id: String,
    pub course_name: String,
    pub title: String,
    pub unread: bool,
}
