use crate::db::DBPool;
use crate::services::unread_announcement_service::{
    HaveUnreadAnnouncementService, UnreadAnnouncementServiceImpl,
};
use std::sync::Arc;

pub trait ServiceManager: HaveUnreadAnnouncementService {}

#[derive(Clone)]
pub struct ServiceManagerImpl {
    unread_announcement_service: UnreadAnnouncementServiceImpl,
}

impl ServiceManager for ServiceManagerImpl {}

impl ServiceManagerImpl {
    pub fn new(db_pool: DBPool) -> Self {
        let pool = Arc::new(db_pool);
        Self {
            unread_announcement_service: UnreadAnnouncementServiceImpl::new(pool.clone()),
        }
    }
}

impl HaveUnreadAnnouncementService for ServiceManagerImpl {
    type Service = UnreadAnnouncementServiceImpl;

    fn unread_announcement_service(&self) -> &Self::Service {
        &self.unread_announcement_service
    }
}