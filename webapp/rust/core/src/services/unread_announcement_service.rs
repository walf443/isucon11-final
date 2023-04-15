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

#[cfg_attr(any(test, feature = "test"), mockall::automock(type Service = MockUnreadAnnouncementServiceVirtual;))]
pub trait HaveUnreadAnnouncementService {
    type Service: UnreadAnnouncementServiceVirtual;
    fn unread_announcement_service(&self) -> &Self::Service;
}

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait UnreadAnnouncementServiceVirtual: Sync {
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

#[async_trait]
pub trait UnreadAnnouncementService:
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
impl<S: UnreadAnnouncementService> UnreadAnnouncementServiceVirtual for S {
    async fn find_all_with_count<'c>(
        &self,
        user_id: &str,
        limit: i64,
        page: i64,
        course_id: Option<&'c str>,
    ) -> Result<(Vec<AnnouncementWithoutDetail>, i64)> {
        UnreadAnnouncementService::find_all_with_count(self, user_id, limit, page, course_id).await
    }

    async fn find_detail_and_mark_read(
        &self,
        announcement_id: &str,
        user_id: &str,
    ) -> Result<AnnouncementDetail> {
        UnreadAnnouncementService::find_detail_and_mark_read(self, announcement_id, user_id).await
    }
}

pub struct UnreadAnnouncementManager {}

#[cfg(test)]
mod tests {
    use crate::db::{get_test_db_conn, DBPool};
    use crate::repos::registration_repository::{
        HaveRegistrationRepository, MockRegistrationRepository,
    };
    use crate::repos::transaction_repository::{
        HaveTransactionRepository, TransactionRepositoryImpl,
    };
    use crate::repos::unread_announcement_repository::{
        HaveUnreadAnnouncementRepository, MockUnreadAnnouncementRepository,
    };
    use crate::services::unread_announcement_service::UnreadAnnouncementService;
    use crate::services::HaveDBPool;

    struct S {
        db_pool: DBPool,
        pub transaction_repo: TransactionRepositoryImpl,
        pub unread_announcement_repo: MockUnreadAnnouncementRepository,
        pub registration_repo: MockRegistrationRepository,
    }
    impl S {
        pub async fn new() -> Self {
            let pool = get_test_db_conn().await.unwrap();
            Self {
                db_pool: pool,
                transaction_repo: TransactionRepositoryImpl {},
                unread_announcement_repo: MockUnreadAnnouncementRepository::new(),
                registration_repo: MockRegistrationRepository::new(),
            }
        }
    }
    impl HaveTransactionRepository for S {
        type Repo = TransactionRepositoryImpl;

        fn transaction_repository(&self) -> &Self::Repo {
            &self.transaction_repo
        }
    }
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

    impl UnreadAnnouncementService for S {}

    mod find_detail_and_mark_read {
        use crate::models::announcement_detail::AnnouncementDetail;
        use crate::repos::error::ReposError::TestError;
        use crate::services::error::Result;
        use crate::services::unread_announcement_service::tests::S;
        use crate::services::unread_announcement_service::UnreadAnnouncementService;

        #[tokio::test]
        #[should_panic(expected = "ReposError(TestError)")]
        async fn find_announcement_detail_by_announcement_id_and_user_id_in_tx_err() -> () {
            let mut service = S::new().await;
            service
                .unread_announcement_repo
                .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
                .returning(|_, _, _| Err(TestError));

            service.find_detail_and_mark_read("", "").await.unwrap();
        }

        #[tokio::test]
        #[should_panic(expected = "AnnouncementNotFound")]
        async fn find_announcement_detail_by_announcement_id_and_user_id_in_tx_none() -> () {
            let mut service = S::new().await;
            service
                .unread_announcement_repo
                .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
                .returning(|_, _, _| Ok(None));

            service.find_detail_and_mark_read("", "").await.unwrap();
        }

        #[tokio::test]
        #[should_panic(expected = "ReposError(TestError)")]
        async fn exist_by_user_id_and_course_id_in_tx_err() -> () {
            let mut service = S::new().await;
            service
                .unread_announcement_repo
                .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
                .returning(|_, _, _| {
                    Ok(Some(AnnouncementDetail {
                        id: "".to_string(),
                        course_id: "".to_string(),
                        course_name: "".to_string(),
                        title: "".to_string(),
                        message: "".to_string(),
                        unread: false,
                    }))
                });

            service
                .registration_repo
                .expect_exist_by_user_id_and_course_id_in_tx()
                .returning(|_, _, _| Err(TestError));

            service.find_detail_and_mark_read("", "").await.unwrap();
        }

        #[tokio::test]
        #[should_panic(expected = "AnnouncementNotFound")]
        async fn exist_by_user_id_and_course_id_in_tx_false() -> () {
            let mut service = S::new().await;
            service
                .unread_announcement_repo
                .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
                .returning(|_, _, _| {
                    Ok(Some(AnnouncementDetail {
                        id: "".to_string(),
                        course_id: "".to_string(),
                        course_name: "".to_string(),
                        title: "".to_string(),
                        message: "".to_string(),
                        unread: false,
                    }))
                });

            service
                .registration_repo
                .expect_exist_by_user_id_and_course_id_in_tx()
                .returning(|_, _, _| Ok(false));

            service.find_detail_and_mark_read("", "").await.unwrap();
        }

        #[tokio::test]
        #[should_panic(expected = "ReposError(TestError)")]
        async fn mark_read_failed() -> () {
            let mut service = S::new().await;
            service
                .unread_announcement_repo
                .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
                .returning(|_, _, _| {
                    Ok(Some(AnnouncementDetail {
                        id: "".to_string(),
                        course_id: "".to_string(),
                        course_name: "".to_string(),
                        title: "".to_string(),
                        message: "".to_string(),
                        unread: false,
                    }))
                });

            service
                .registration_repo
                .expect_exist_by_user_id_and_course_id_in_tx()
                .returning(|_, _, _| Ok(true));

            service
                .unread_announcement_repo
                .expect_mark_read()
                .returning(|_, _, _| Err(TestError));

            service.find_detail_and_mark_read("", "").await.unwrap();
        }

        #[tokio::test]
        async fn success_case() -> Result<()> {
            let mut service = S::new().await;
            let expected = AnnouncementDetail {
                id: "aid".to_string(),
                course_id: "course_id".to_string(),
                course_name: "course_name".to_string(),
                title: "title".to_string(),
                message: "message".to_string(),
                unread: true,
            };
            let detail = expected.clone();

            service
                .unread_announcement_repo
                .expect_find_announcement_detail_by_announcement_id_and_user_id_in_tx()
                .withf(|_, aid, user_id| aid == "aid" && user_id == "user_id")
                .returning(move |_, _, _| Ok(Some(detail.clone())));

            service
                .registration_repo
                .expect_exist_by_user_id_and_course_id_in_tx()
                .withf(|_, user_id, course_id| user_id == "user_id" && course_id == "course_id")
                .returning(|_, _, _| Ok(true));

            service
                .unread_announcement_repo
                .expect_mark_read()
                .withf(|_, aid, user_id| aid == "aid" && user_id == "user_id")
                .returning(|_, _, _| Ok(()));

            let detail = service
                .find_detail_and_mark_read("aid", "user_id")
                .await
                .unwrap();

            assert_eq!(detail, expected);

            Ok(())
        }
    }
}
