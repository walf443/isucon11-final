use std::sync::Arc;
use isucholar_core::db::DBPool;
use isucholar_core::repos::announcement_repository::HaveAnnouncementRepository;
use isucholar_core::repos::course_repository::HaveCourseRepository;
use isucholar_core::repos::registration_repository::HaveRegistrationRepository;
use isucholar_core::repos::transaction_repository::{HaveTransactionRepository, TransactionRepositoryImpl};
use isucholar_core::repos::unread_announcement_repository::HaveUnreadAnnouncementRepository;
use isucholar_core::services::announcement_service::AnnouncementServiceImpl;
use isucholar_core::services::HaveDBPool;
use crate::repos::announcement_repository::AnnouncementRepositoryImpl;
use crate::repos::course_repository::CourseRepositoryImpl;
use crate::repos::registration_repository::RegistrationRepositoryImpl;
use crate::repos::unread_announcement_repository::UnreadAnnouncementRepositoryImpl;

#[derive(Clone)]
pub struct AnnouncementServiceInfra {
    db_pool: Arc<DBPool>,
    transaction: TransactionRepositoryImpl,
    announcement: AnnouncementRepositoryImpl,
    course: CourseRepositoryImpl,
    registration: RegistrationRepositoryImpl,
    unread_announcement: UnreadAnnouncementRepositoryImpl,
}

impl AnnouncementServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            transaction: TransactionRepositoryImpl {},
            announcement: AnnouncementRepositoryImpl {},
            course: CourseRepositoryImpl {},
            registration: RegistrationRepositoryImpl {},
            unread_announcement: UnreadAnnouncementRepositoryImpl {},
        }
    }
}

impl HaveDBPool for AnnouncementServiceInfra {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}

impl HaveTransactionRepository for AnnouncementServiceInfra {
    type Repo = TransactionRepositoryImpl;

    fn transaction_repository(&self) -> &Self::Repo {
        &self.transaction
    }
}

impl HaveCourseRepository for AnnouncementServiceInfra {
    type Repo = CourseRepositoryImpl;

    fn course_repo(&self) -> &Self::Repo {
        &self.course
    }
}

impl HaveAnnouncementRepository for AnnouncementServiceInfra {
    type Repo = AnnouncementRepositoryImpl;

    fn announcement_repo(&self) -> &Self::Repo {
        &self.announcement
    }
}

impl HaveRegistrationRepository for AnnouncementServiceInfra {
    type Repo = RegistrationRepositoryImpl;

    fn registration_repo(&self) -> &Self::Repo {
        &self.registration
    }
}

impl HaveUnreadAnnouncementRepository for AnnouncementServiceInfra {
    type Repo = UnreadAnnouncementRepositoryImpl;

    fn unread_announcement_repo(&self) -> &Self::Repo {
        &self.unread_announcement
    }
}

impl AnnouncementServiceImpl for AnnouncementServiceInfra {}