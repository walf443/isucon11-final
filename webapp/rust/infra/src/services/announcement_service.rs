use crate::repos::announcement_repository::AnnouncementRepositoryInfra;
use crate::repos::course_repository::CourseRepositoryInfra;
use crate::repos::registration_repository::RegistrationRepositoryInfra;
use crate::repos::unread_announcement_repository::UnreadAnnouncementRepositoryInfra;
use isucholar_core::db::DBPool;
use isucholar_core::repos::announcement_repository::HaveAnnouncementRepository;
use isucholar_core::repos::course_repository::HaveCourseRepository;
use isucholar_core::repos::registration_repository::HaveRegistrationRepository;
use isucholar_core::repos::unread_announcement_repository::HaveUnreadAnnouncementRepository;
use isucholar_core::services::announcement_service::AnnouncementServiceImpl;
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AnnouncementServiceInfra {
    db_pool: Arc<DBPool>,
    announcement: AnnouncementRepositoryInfra,
    course: CourseRepositoryInfra,
    registration: RegistrationRepositoryInfra,
    unread_announcement: UnreadAnnouncementRepositoryInfra,
}

impl AnnouncementServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            announcement: AnnouncementRepositoryInfra {},
            course: CourseRepositoryInfra {},
            registration: RegistrationRepositoryInfra {},
            unread_announcement: UnreadAnnouncementRepositoryInfra {},
        }
    }
}

impl AnnouncementServiceImpl for AnnouncementServiceInfra {}

impl HaveDBPool for AnnouncementServiceInfra {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}

impl HaveCourseRepository for AnnouncementServiceInfra {
    type Repo = CourseRepositoryInfra;

    fn course_repo(&self) -> &Self::Repo {
        &self.course
    }
}

impl HaveAnnouncementRepository for AnnouncementServiceInfra {
    type Repo = AnnouncementRepositoryInfra;

    fn announcement_repo(&self) -> &Self::Repo {
        &self.announcement
    }
}

impl HaveRegistrationRepository for AnnouncementServiceInfra {
    type Repo = RegistrationRepositoryInfra;

    fn registration_repo(&self) -> &Self::Repo {
        &self.registration
    }
}

impl HaveUnreadAnnouncementRepository for AnnouncementServiceInfra {
    type Repo = UnreadAnnouncementRepositoryInfra;

    fn unread_announcement_repo(&self) -> &Self::Repo {
        &self.unread_announcement
    }
}
