use crate::db::DBPool;
use crate::models::announcement_detail::AnnouncementDetail;
use crate::repos::registration_repository::{
    HaveRegistrationRepository, RegistrationRepository, RegistrationRepositoryImpl,
};
use crate::repos::transaction_repository::{
    HaveTransactionRepository, TransactionRepository, TransactionRepositoryImpl,
};
use crate::repos::unread_announcement_repository::{
    HaveUnreadAnnouncementRepository, UnreadAnnouncementRepository,
    UnreadAnnouncementRepositoryImpl,
};
use crate::services::error::Error::AnnouncementNotFound;
use crate::services::error::Result;
use async_trait::async_trait;

pub trait HaveUnreadAnnounceService {
    type Service: UnreadAnnouncementService;
    fn unread_announcement_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait UnreadAnnouncementService:
    Sync + HaveTransactionRepository + HaveUnreadAnnouncementRepository + HaveRegistrationRepository
{
    async fn find_detail_and_mark_read(
        &self,
        pool: &DBPool,
        announcement_id: &str,
        user_id: &str,
    ) -> Result<AnnouncementDetail> {
        let mut tx = self.transaction_repository().begin(pool).await?;

        let announcement = self
            .unread_announcement_repo()
            .find_announcement_detail_by_announcement_id_and_user_id_in_tx(
                &mut tx,
                announcement_id,
                user_id,
            )
            .await?;

        if announcement.is_none() {
            return Err(AnnouncementNotFound);
        }
        let announcement = announcement.unwrap();

        let is_exist = self
            .registration_repo()
            .exist_by_user_id_and_course_id_in_tx(&mut tx, user_id, &announcement.course_id)
            .await?;

        if !is_exist {
            return Err(AnnouncementNotFound);
        }

        self.unread_announcement_repo()
            .mark_read(&mut tx, announcement_id, user_id)
            .await?;

        tx.commit().await?;

        Ok(announcement)
    }
}

pub struct UnreadAnnouncementManager {}

pub struct UnreadAnnouncementServiceImpl {
    transaction: TransactionRepositoryImpl,
    unread_announcement: UnreadAnnouncementRepositoryImpl,
    registration: RegistrationRepositoryImpl,
}

impl UnreadAnnouncementServiceImpl {
    pub fn new() -> Self {
        Self {
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

impl UnreadAnnouncementService for UnreadAnnouncementServiceImpl {}
