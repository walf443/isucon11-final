use crate::db::{get_test_db_conn, DBPool};
use crate::repos::registration_repository::{
    HaveRegistrationRepository, MockRegistrationRepository,
};
use crate::repos::unread_announcement_repository::{
    HaveUnreadAnnouncementRepository, MockUnreadAnnouncementRepository,
};
use crate::services::unread_announcement_service::UnreadAnnouncementServiceImpl;
use crate::services::HaveDBPool;

pub(crate) struct S {
    db_pool: DBPool,
    pub unread_announcement_repo: MockUnreadAnnouncementRepository,
    pub registration_repo: MockRegistrationRepository,
}
impl S {
    pub async fn new() -> Self {
        let pool = get_test_db_conn().await.unwrap();
        Self {
            db_pool: pool,
            unread_announcement_repo: MockUnreadAnnouncementRepository::new(),
            registration_repo: MockRegistrationRepository::new(),
        }
    }
}

impl UnreadAnnouncementServiceImpl for S {}

impl HaveUnreadAnnouncementRepository for S {
    type Repo = MockUnreadAnnouncementRepository;

    fn unread_announcement_repo(&self) -> &Self::Repo {
        &self.unread_announcement_repo
    }
}
impl HaveRegistrationRepository for S {
    type Repo = MockRegistrationRepository;

    fn registration_repo(&self) -> &Self::Repo {
        &self.registration_repo
    }
}

impl HaveDBPool for S {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}

mod find_all_with_count;
mod find_detail_and_mark_read;
