use crate::db::{DBConn, TxConn};
use crate::models::announcement::Announcement;
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AnnouncementRepository {
    async fn create_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        announcement: &Announcement,
    ) -> Result<()>;
    async fn find_by_id(&self, conn: &mut DBConn, id: &str) -> Result<Announcement>;
}

pub trait HaveAnnouncementRepository {
    type Repo: Sync + AnnouncementRepository;
    fn announcement_repo(&self) -> &Self::Repo;
}
