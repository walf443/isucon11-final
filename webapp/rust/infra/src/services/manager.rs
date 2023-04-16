use crate::services::announcement_service::AnnouncementServiceInfra;
use crate::services::unread_announcement_service::UnreadAnnouncementServiceInfra;
use isucholar_core::db::DBPool;
use isucholar_core::services::announcement_service::HaveAnnouncementService;
use isucholar_core::services::manager::ServiceManager;
use isucholar_core::services::unread_announcement_service::HaveUnreadAnnouncementService;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServiceManagerImpl {
    announcement_service: AnnouncementServiceInfra,
    unread_announcement_service: UnreadAnnouncementServiceInfra,
}

impl HaveAnnouncementService for ServiceManagerImpl {
    type Service = AnnouncementServiceInfra;

    fn announcement_service(&self) -> &Self::Service {
        &self.announcement_service
    }
}

impl ServiceManager for ServiceManagerImpl {}

impl ServiceManagerImpl {
    pub fn new(db_pool: DBPool) -> Self {
        let pool = Arc::new(db_pool);
        Self {
            announcement_service: AnnouncementServiceInfra::new(pool.clone()),
            unread_announcement_service: UnreadAnnouncementServiceInfra::new(pool.clone()),
        }
    }
}

impl HaveUnreadAnnouncementService for ServiceManagerImpl {
    type Service = UnreadAnnouncementServiceInfra;

    fn unread_announcement_service(&self) -> &Self::Service {
        &self.unread_announcement_service
    }
}
