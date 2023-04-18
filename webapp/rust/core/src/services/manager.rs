use crate::services::announcement_service::HaveAnnouncementService;
use crate::services::class_service::HaveClassService;
use crate::services::course_service::HaveCourseService;
use crate::services::unread_announcement_service::HaveUnreadAnnouncementService;
use crate::services::user_service::HaveUserService;

pub trait ServiceManager:
    HaveUnreadAnnouncementService
    + HaveAnnouncementService
    + HaveUserService
    + HaveCourseService
    + HaveClassService
{
}
