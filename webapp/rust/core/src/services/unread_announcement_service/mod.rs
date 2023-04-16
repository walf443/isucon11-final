use crate::models::announcement::AnnouncementWithoutDetail;
use crate::models::announcement_detail::AnnouncementDetail;
use crate::repos::registration_repository::{HaveRegistrationRepository, RegistrationRepository};
use crate::repos::transaction_repository::{HaveTransactionRepository, TransactionRepository};
use crate::repos::unread_announcement_repository::{
    HaveUnreadAnnouncementRepository, UnreadAnnouncementRepository,
};
use crate::services::error::Error::AnnouncementNotFound;
use crate::services::error::Result;
use crate::services::HaveDBPool;
use async_trait::async_trait;

#[cfg(test)]
mod tests;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait UnreadAnnouncementService: Sync {
    async fn find_all_with_count<'c>(
        &self,
        user_id: &str,
        limit: i64,
        page: i64,
        course_id: Option<&'c str>,
    ) -> Result<(Vec<AnnouncementWithoutDetail>, i64)>;

    async fn find_detail_and_mark_read(
        &self,
        announcement_id: &str,
        user_id: &str,
    ) -> Result<AnnouncementDetail>;
}

#[cfg_attr(any(test, feature = "test"), mockall::automock(type Service = MockUnreadAnnouncementService;))]
pub trait HaveUnreadAnnouncementService {
    type Service: UnreadAnnouncementService;
    fn unread_announcement_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait UnreadAnnouncementServiceImpl:
    Sync
    + HaveDBPool
    + HaveTransactionRepository
    + HaveUnreadAnnouncementRepository
    + HaveRegistrationRepository
{
    async fn find_all_with_count<'c>(
        &self,
        user_id: &str,
        limit: i64,
        page: i64,
        course_id: Option<&'c str>,
    ) -> Result<(Vec<AnnouncementWithoutDetail>, i64)> {
        let pool = self.get_db_pool();
        let mut tx = self.transaction_repository().begin(pool).await?;
        let offset = limit * (page - 1);

        let repo = self.unread_announcement_repo();
        let announcements = repo
            .find_unread_announcements_by_user_id_in_tx(&mut tx, &user_id, limit, offset, course_id)
            .await?;

        let unread_count = repo
            .count_unread_by_user_id_in_tx(&mut tx, &user_id)
            .await?;

        tx.commit().await?;

        Ok((announcements, unread_count))
    }

    async fn find_detail_and_mark_read(
        &self,
        announcement_id: &str,
        user_id: &str,
    ) -> Result<AnnouncementDetail> {
        let pool = self.get_db_pool();
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

#[async_trait]
impl<S: UnreadAnnouncementServiceImpl> UnreadAnnouncementService for S {
    async fn find_all_with_count<'c>(
        &self,
        user_id: &str,
        limit: i64,
        page: i64,
        course_id: Option<&'c str>,
    ) -> Result<(Vec<AnnouncementWithoutDetail>, i64)> {
        UnreadAnnouncementServiceImpl::find_all_with_count(self, user_id, limit, page, course_id)
            .await
    }

    async fn find_detail_and_mark_read(
        &self,
        announcement_id: &str,
        user_id: &str,
    ) -> Result<AnnouncementDetail> {
        UnreadAnnouncementServiceImpl::find_detail_and_mark_read(self, announcement_id, user_id)
            .await
    }
}

pub struct UnreadAnnouncementManager {}

