use crate::models::announcement::AnnouncementID;
use crate::models::course::CourseID;

#[derive(Debug, sqlx::FromRow, serde::Serialize, PartialEq, Clone)]
pub struct AnnouncementDetail {
    pub id: AnnouncementID,
    pub course_id: CourseID,
    pub course_name: String,
    pub title: String,
    pub message: String,
    pub unread: bool,
}

impl AnnouncementDetail {}
