use crate::models::course::CourseID;
use fake::Dummy;
use kubetsu::Id;
use std::fmt::Debug;

#[derive(Debug, Clone, sqlx::FromRow, PartialEq, Eq, Dummy)]
pub struct Announcement {
    pub id: Id<Self, String>,
    pub course_id: CourseID,
    pub title: String,
    pub message: String,
}

pub type AnnouncementID = Id<Announcement, String>;

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct AnnouncementWithoutDetail {
    pub id: AnnouncementID,
    pub course_id: CourseID,
    pub course_name: String,
    pub title: String,
    pub unread: bool,
}
