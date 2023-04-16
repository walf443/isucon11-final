use crate::services::announcement_service::HaveAnnouncementService;
use crate::services::unread_announcement_service::HaveUnreadAnnouncementService;

pub trait ServiceManager: HaveUnreadAnnouncementService + HaveAnnouncementService {}
