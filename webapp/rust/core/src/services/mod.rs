use crate::db::DBPool;

pub mod error;
pub mod manager;
pub mod unread_announcement_service;

pub trait HaveDBPool {
    fn get_db_pool(&self) -> &DBPool;
}
