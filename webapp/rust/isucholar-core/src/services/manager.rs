use crate::services::unread_announcement_service::{
    HaveUnreadAnnouncementService, UnreadAnnouncementServiceImpl,
};

pub trait ServiceManager: HaveUnreadAnnouncementService {}

pub struct ServiceManagerImpl {
    unread_announcement_service: UnreadAnnouncementServiceImpl,
}

impl ServiceManager for ServiceManagerImpl {}

impl ServiceManagerImpl {
    pub fn new() -> Self {
        Self {
            unread_announcement_service: UnreadAnnouncementServiceImpl::new(),
        }
    }
}

impl HaveUnreadAnnouncementService for ServiceManagerImpl {
    type Service = UnreadAnnouncementServiceImpl;

    fn unread_announcement_service(&self) -> &Self::Service {
        &self.unread_announcement_service
    }
}
