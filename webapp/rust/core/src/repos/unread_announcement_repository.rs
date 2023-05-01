use crate::db::{DBConn, TxConn};
use crate::models::announcement::AnnouncementWithoutDetail;
use crate::models::announcement_detail::AnnouncementDetail;
use crate::repos::error::Result;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait UnreadAnnouncementRepository {
    async fn create<'c>(
        &self,
        conn: &mut DBConn,
        announcement_id: &str,
        user_id: &str,
    ) -> Result<()>;
    async fn mark_read<'c>(&self, tx: &mut TxConn<'c>, id: &str, user_id: &str) -> Result<()>;
    async fn count_unread_by_user_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
    ) -> Result<i64>;
    async fn find_unread_announcements_by_user_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &'c str,
        limit: i64,
        offset: i64,
        course_id: Option<&'c str>,
    ) -> Result<Vec<AnnouncementWithoutDetail>>;
    async fn find_announcement_detail_by_announcement_id_and_user_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        announcement_id: &str,
        user_id: &str,
    ) -> Result<Option<AnnouncementDetail>>;
}

pub trait HaveUnreadAnnouncementRepository {
    type Repo: UnreadAnnouncementRepository + Sync;
    fn unread_announcement_repo(&self) -> &Self::Repo;
}
