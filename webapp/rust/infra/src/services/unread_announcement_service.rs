use crate::repos::registration_repository::RegistrationRepositoryImpl;
use crate::repos::unread_announcement_repository::UnreadAnnouncementRepositoryImpl;
use isucholar_core::db::DBPool;
use isucholar_core::repos::registration_repository::HaveRegistrationRepository;
use isucholar_core::repos::transaction_repository::{
    HaveTransactionRepository, TransactionRepositoryImpl,
};
use isucholar_core::repos::unread_announcement_repository::HaveUnreadAnnouncementRepository;
use isucholar_core::services::unread_announcement_service::UnreadAnnouncementService;
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct UnreadAnnouncementServiceImpl {
    db_pool: Arc<DBPool>,
    transaction: TransactionRepositoryImpl,
    unread_announcement: UnreadAnnouncementRepositoryImpl,
    registration: RegistrationRepositoryImpl,
}

impl UnreadAnnouncementServiceImpl {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            transaction: TransactionRepositoryImpl {},
            unread_announcement: UnreadAnnouncementRepositoryImpl {},
            registration: RegistrationRepositoryImpl {},
        }
    }
}

impl HaveTransactionRepository for UnreadAnnouncementServiceImpl {
    type Repo = TransactionRepositoryImpl;

    fn transaction_repository(&self) -> &Self::Repo {
        &self.transaction
    }
}

impl HaveUnreadAnnouncementRepository for UnreadAnnouncementServiceImpl {
    type Repo = UnreadAnnouncementRepositoryImpl;

    fn unread_announcement_repo(&self) -> &Self::Repo {
        &self.unread_announcement
    }
}

impl HaveRegistrationRepository for UnreadAnnouncementServiceImpl {
    type Repo = RegistrationRepositoryImpl;

    fn registration_repo(&self) -> &Self::Repo {
        &self.registration
    }
}

impl HaveDBPool for UnreadAnnouncementServiceImpl {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}

impl UnreadAnnouncementService for UnreadAnnouncementServiceImpl {}
