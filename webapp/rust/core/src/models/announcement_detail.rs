use crate::models::announcement::AnnouncementID;

#[derive(Debug, sqlx::FromRow, serde::Serialize, PartialEq, Clone)]
pub struct AnnouncementDetail {
    pub id: AnnouncementID,
    pub course_id: String,
    pub course_name: String,
    pub title: String,
    pub message: String,
    pub unread: bool,
}

impl AnnouncementDetail {}
