use crate::db::DBPool;

pub mod announcement_service;
pub mod error;
pub mod manager;
pub mod unread_announcement_service;

pub trait HaveDBPool {
    fn get_db_pool(&self) -> &DBPool;
}
