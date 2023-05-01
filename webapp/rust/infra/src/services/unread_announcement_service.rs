use crate::repos::registration_repository::RegistrationRepositoryInfra;
use crate::repos::unread_announcement_repository::UnreadAnnouncementRepositoryInfra;
use isucholar_core::db::DBPool;
use isucholar_core::repos::registration_repository::HaveRegistrationRepository;
use isucholar_core::repos::unread_announcement_repository::HaveUnreadAnnouncementRepository;
use isucholar_core::services::unread_announcement_service::UnreadAnnouncementServiceImpl;
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct UnreadAnnouncementServiceInfra {
    db_pool: Arc<DBPool>,
    unread_announcement: UnreadAnnouncementRepositoryInfra,
    registration: RegistrationRepositoryInfra,
}

impl UnreadAnnouncementServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            unread_announcement: UnreadAnnouncementRepositoryInfra {},
            registration: RegistrationRepositoryInfra {},
        }
    }
}

impl UnreadAnnouncementServiceImpl for UnreadAnnouncementServiceInfra {}

impl HaveUnreadAnnouncementRepository for UnreadAnnouncementServiceInfra {
    type Repo = UnreadAnnouncementRepositoryInfra;

    fn unread_announcement_repo(&self) -> &Self::Repo {
        &self.unread_announcement
    }
}

impl HaveRegistrationRepository for UnreadAnnouncementServiceInfra {
    type Repo = RegistrationRepositoryInfra;

    fn registration_repo(&self) -> &Self::Repo {
        &self.registration
    }
}

impl HaveDBPool for UnreadAnnouncementServiceInfra {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}
