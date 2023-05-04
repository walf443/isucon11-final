use crate::models::announcement::{AnnouncementID, AnnouncementWithoutDetail};
use crate::models::announcement_detail::AnnouncementDetail;
use crate::models::course::CourseID;
use crate::models::user::UserID;
use crate::repos::registration_repository::{HaveRegistrationRepository, RegistrationRepository};
use crate::repos::unread_announcement_repository::{
    HaveUnreadAnnouncementRepository, UnreadAnnouncementRepository,
};
use crate::services::error::Error::AnnouncementNotFound;
use crate::services::error::Result;
use crate::services::HaveDBPool;
use async_trait::async_trait;

mod find_all_with_count;
mod find_detail_and_mark_read;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait UnreadAnnouncementService: Sync {
    async fn find_all_with_count(
        &self,
        user_id: &UserID,
        limit: i64,
        page: i64,
        course_id: Option<CourseID>,
    ) -> Result<(Vec<AnnouncementWithoutDetail>, i64)>;

    async fn find_detail_and_mark_read(
        &self,
        announcement_id: &AnnouncementID,
        user_id: &UserID,
    ) -> Result<AnnouncementDetail>;
}

#[cfg_attr(any(test, feature = "test"), mockall::automock(type Service = MockUnreadAnnouncementService;))]
pub trait HaveUnreadAnnouncementService {
    type Service: UnreadAnnouncementService;
    fn unread_announcement_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait UnreadAnnouncementServiceImpl:
    Sync + HaveDBPool + HaveUnreadAnnouncementRepository + HaveRegistrationRepository
{
    async fn find_all_with_count(
        &self,
        user_id: &UserID,
        limit: i64,
        page: i64,
        course_id: Option<CourseID>,
    ) -> Result<(Vec<AnnouncementWithoutDetail>, i64)> {
        let pool = self.get_db_pool();
        let mut tx = pool.begin().await?;

        let offset = limit * (page - 1);

        let repo = self.unread_announcement_repo();
        let announcements = repo
            .find_unread_announcements_by_user_id(&mut tx, &user_id, limit, offset, course_id)
            .await?;

        let unread_count = repo.count_unread_by_user_id(&mut tx, &user_id).await?;

        tx.commit().await?;

        Ok((announcements, unread_count))
    }

    async fn find_detail_and_mark_read(
        &self,
        announcement_id: &AnnouncementID,
        user_id: &UserID,
    ) -> Result<AnnouncementDetail> {
        let pool = self.get_db_pool();
        let mut tx = pool.begin().await?;

        let announcement = self
            .unread_announcement_repo()
            .find_announcement_detail_by_announcement_id_and_user_id(
                &mut tx,
                &announcement_id,
                &user_id,
            )
            .await?;

        if announcement.is_none() {
            return Err(AnnouncementNotFound);
        }
        let announcement = announcement.unwrap();

        let is_exist = self
            .registration_repo()
            .exist_by_user_id_and_course_id(&mut tx, &user_id, &announcement.course_id)
            .await?;

        if !is_exist {
            return Err(AnnouncementNotFound);
        }

        self.unread_announcement_repo()
            .mark_read(&mut tx, &announcement_id, &user_id)
            .await?;

        tx.commit().await?;

        Ok(announcement)
    }
}

#[async_trait]
impl<S: UnreadAnnouncementServiceImpl> UnreadAnnouncementService for S {
    async fn find_all_with_count(
        &self,
        user_id: &UserID,
        limit: i64,
        page: i64,
        course_id: Option<CourseID>,
    ) -> Result<(Vec<AnnouncementWithoutDetail>, i64)> {
        UnreadAnnouncementServiceImpl::find_all_with_count(self, user_id, limit, page, course_id)
            .await
    }

    async fn find_detail_and_mark_read(
        &self,
        announcement_id: &AnnouncementID,
        user_id: &UserID,
    ) -> Result<AnnouncementDetail> {
        UnreadAnnouncementServiceImpl::find_detail_and_mark_read(self, announcement_id, user_id)
            .await
    }
}

#[cfg(test)]
mod tests {
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
}
