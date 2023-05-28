use crate::models::course::CourseID;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, sqlx::FromRow, PartialEq, Eq, Dummy)]
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
}

impl Display for AnnouncementID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct AnnouncementWithoutDetail {
    pub id: AnnouncementID,
    pub course_id: CourseID,
    pub course_name: String,
    pub title: String,
    pub unread: bool,
}
