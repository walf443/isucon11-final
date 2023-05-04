use crate::db::DBPool;

pub mod announcement_service;
pub mod class_service;
pub mod course_service;
pub mod error;
pub mod grade_summary_service;
pub mod manager;
pub mod registration_course_service;
pub mod submission_service;
pub mod unread_announcement_service;
pub mod user_service;

pub trait HaveDBPool {
    fn get_db_pool(&self) -> &DBPool;
}
