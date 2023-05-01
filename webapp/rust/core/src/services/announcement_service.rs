use crate::models::announcement::Announcement;
use crate::repos::announcement_repository::{AnnouncementRepository, HaveAnnouncementRepository};
use crate::repos::course_repository::{CourseRepository, HaveCourseRepository};
use crate::repos::error::ReposError;
use crate::repos::registration_repository::{HaveRegistrationRepository, RegistrationRepository};
use crate::repos::unread_announcement_repository::{
    HaveUnreadAnnouncementRepository, UnreadAnnouncementRepository,
};
use crate::services::error::Error::{AnnouncementDuplicate, CourseNotFound};
use crate::services::error::Result;
use crate::services::HaveDBPool;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait AnnouncementService: Sync {
    async fn create(&self, announcement: &Announcement) -> Result<()>;
}

#[cfg_attr(any(test, feature = "test"), mockall::automock(type Service = MockAnnouncementService;))]
pub trait HaveAnnouncementService {
    type Service: AnnouncementService;
    fn announcement_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait AnnouncementServiceImpl:
    Sync
    + HaveDBPool
    + HaveCourseRepository
    + HaveAnnouncementRepository
    + HaveRegistrationRepository
    + HaveUnreadAnnouncementRepository
{
    async fn create(&self, announcement: &Announcement) -> Result<()> {
        let pool = self.get_db_pool();
        let mut tx = pool.begin().await?;

        let is_exist = self
            .course_repo()
            .exist_by_id(&mut tx, &announcement.course_id)
            .await?;
        if !is_exist {
            return Err(CourseNotFound);
        }

        let result = self
            .announcement_repo()
            .create(&mut tx, &announcement)
            .await;

        match result {
            Ok(_) => {}
            Err(e) => {
                let _ = tx.rollback().await;
                match e {
                    ReposError::AnnouncementDuplicate => {
                        let mut conn = pool.acquire().await?;
                        let an = self
                            .announcement_repo()
                            .find_by_id(&mut conn, &announcement.id)
                            .await?;
                        if announcement.course_id != an.course_id
                            || announcement.title != an.title
                            || announcement.message != an.message
                        {
                            return Err(AnnouncementDuplicate);
                        } else {
                            return Ok(());
                        }
                    }
                    _ => return Err(e.into()),
                }
            }
        }

        let targets = self
            .registration_repo()
            .find_users_by_course_id(&mut tx, &announcement.course_id)
            .await?;

        let repo = self.unread_announcement_repo();
        for user in targets {
            repo.create(&mut tx, &announcement.id, &user.id)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl<S: AnnouncementServiceImpl> AnnouncementService for S {
    async fn create(&self, announcement: &Announcement) -> Result<()> {
        AnnouncementServiceImpl::create(self, announcement).await
    }
}
