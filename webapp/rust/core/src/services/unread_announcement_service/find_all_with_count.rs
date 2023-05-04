#[cfg(test)]
mod tests {
    use crate::models::user::UserID;
    use crate::repos::error::ReposError::TestError;
    use crate::services::unread_announcement_service::tests_tmp::S;
    use crate::services::unread_announcement_service::UnreadAnnouncementServiceImpl;
    use fake::{Fake, Faker};

    #[tokio::test]
    #[should_panic(expected = "ReposError(TestError)")]
    async fn find_unread_announcements_by_user_id_failed() -> () {
        let mut service = S::new().await;
        let user_id: UserID = Faker.fake();

        service
            .unread_announcement_repo
            .expect_find_unread_announcements_by_user_id()
            .withf(|_, _, limit, offset, _| *limit == 5 && *offset == 5)
            .returning(|_, _, _, _, _| Err(TestError));

        service
            .find_all_with_count(&user_id, 5, 2, None)
            .await
            .unwrap();
    }
    #[tokio::test]
    #[should_panic(expected = "ReposError(TestError)")]
    async fn count_unread_by_user_id_failed() -> () {
        let mut service = S::new().await;
        let user_id: UserID = Faker.fake();

        service
            .unread_announcement_repo
            .expect_find_unread_announcements_by_user_id()
            .returning(|_, _, _, _, _| Ok(Vec::new()));

        let id = user_id.clone();
        service
            .unread_announcement_repo
            .expect_count_unread_by_user_id()
            .withf(move |_, uid| uid.to_string() == id.to_string())
            .returning(|_, _| Err(TestError));

        service
            .find_all_with_count(&user_id, 1, 1, None)
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn success_case() -> () {
        let mut service = S::new().await;
        let user_id: UserID = Faker.fake();

        service
            .unread_announcement_repo
            .expect_find_unread_announcements_by_user_id()
            .returning(|_, _, _, _, _| Ok(Vec::new()));

        let id = user_id.clone();
        service
            .unread_announcement_repo
            .expect_count_unread_by_user_id()
            .withf(move |_, uid| uid.to_string() == id.to_string())
            .returning(|_, _| Ok(1));

        let result = service
            .find_all_with_count(&user_id, 1, 1, None)
            .await
            .unwrap();
        assert_eq!(result.0.len(), 0);
        assert_eq!(result.1, 1);
    }
}
