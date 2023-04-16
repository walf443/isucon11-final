use crate::repos::registration_repository::RegistrationRepositoryImpl;
use crate::repos::unread_announcement_repository::UnreadAnnouncementRepositoryImpl;
use isucholar_core::db::DBPool;
use isucholar_core::repos::registration_repository::HaveRegistrationRepository;
use isucholar_core::repos::transaction_repository::{
    HaveTransactionRepository, TransactionRepositoryImpl,
};
use isucholar_core::repos::unread_announcement_repository::HaveUnreadAnnouncementRepository;
use isucholar_core::services::unread_announcement_service::UnreadAnnouncementServiceImpl;
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct UnreadAnnouncementServiceInfra {
    db_pool: Arc<DBPool>,
    transaction: TransactionRepositoryImpl,
    unread_announcement: UnreadAnnouncementRepositoryImpl,
    registration: RegistrationRepositoryImpl,
}

impl UnreadAnnouncementServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            transaction: TransactionRepositoryImpl {},
            unread_announcement: UnreadAnnouncementRepositoryImpl {},
            registration: RegistrationRepositoryImpl {},
        }
    }
}

impl UnreadAnnouncementServiceImpl for UnreadAnnouncementServiceInfra {}

impl HaveTransactionRepository for UnreadAnnouncementServiceInfra {
    type Repo = TransactionRepositoryImpl;

    fn transaction_repository(&self) -> &Self::Repo {
        &self.transaction
    }
}

impl HaveUnreadAnnouncementRepository for UnreadAnnouncementServiceInfra {
    type Repo = UnreadAnnouncementRepositoryImpl;

    fn unread_announcement_repo(&self) -> &Self::Repo {
        &self.unread_announcement
    }
}

impl HaveRegistrationRepository for UnreadAnnouncementServiceInfra {
    type Repo = RegistrationRepositoryImpl;

    fn registration_repo(&self) -> &Self::Repo {
        &self.registration
    }
}

impl HaveDBPool for UnreadAnnouncementServiceInfra {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}
