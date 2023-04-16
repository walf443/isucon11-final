use crate::services::announcement_service::HaveAnnouncementService;
use crate::services::unread_announcement_service::HaveUnreadAnnouncementService;
use crate::services::user_service::HaveUserService;

pub trait ServiceManager:
    HaveUnreadAnnouncementService + HaveAnnouncementService + HaveUserService
{
}
