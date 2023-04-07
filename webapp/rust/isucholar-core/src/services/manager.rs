use crate::services::unread_announcement_service::{
    HaveUnreadAnnounceService, UnreadAnnouncementServiceImpl,
};

pub trait ServiceManager: HaveUnreadAnnounceService {}

pub struct ServiceManagerImpl {
    unread_announcement_service: UnreadAnnouncementServiceImpl,
}

impl ServiceManagerImpl {
    pub fn new() -> Self {
        Self {
            unread_announcement_service: UnreadAnnouncementServiceImpl::new(),
        }
    }
}

impl HaveUnreadAnnounceService for ServiceManagerImpl {
    type Service = UnreadAnnouncementServiceImpl;

    fn unread_announcement_service(&self) -> &Self::Service {
        &self.unread_announcement_service
    }
}

impl ServiceManager for ServiceManagerImpl {}
