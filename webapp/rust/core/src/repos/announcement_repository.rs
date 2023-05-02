use crate::db::DBConn;
use crate::models::announcement::{Announcement, AnnouncementID};
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AnnouncementRepository {
    async fn create(&self, conn: &mut DBConn, announcement: &Announcement) -> Result<()>;
    async fn find_by_id(&self, conn: &mut DBConn, id: &AnnouncementID) -> Result<Announcement>;
}

pub trait HaveAnnouncementRepository {
    type Repo: Sync + AnnouncementRepository;
    fn announcement_repo(&self) -> &Self::Repo;
}
