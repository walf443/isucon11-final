#[cfg(test)]
mod tests {
    use crate::db::get_test_db_conn;
    use crate::models::announcement::Announcement;
    use crate::models::user::User;
    use crate::repos::error::ReposError::AnnouncementDuplicate;
    use crate::repos::manager::tests::MockRepositoryManager;
    use crate::services::announcement_service::AnnouncementService;
    use fake::{Fake, Faker};

    #[tokio::test]
    #[should_panic(expected = "CourseNotFound")]
    async fn invalid_course_id_case() {
        let conn = get_test_db_conn().await.unwrap();

        let mut service = MockRepositoryManager::new(conn);
        let announcement: Announcement = Faker.fake();

        let cid = announcement.course_id.clone();
        service
            .course_repo
            .expect_exist_by_id()
            .withf(move |_, course_id| course_id == &cid)
            .returning(|_, _| Ok(false));

        service.create(&announcement).await.unwrap();
    }

    #[tokio::test]
    async fn duplicate_success_case() {
        let conn = get_test_db_conn().await.unwrap();

        let mut service = MockRepositoryManager::new(conn);
        let announcement: Announcement = Faker.fake();

        let cid = announcement.course_id.clone();
        service
            .course_repo
            .expect_exist_by_id()
            .withf(move |_, course_id| course_id == &cid.clone())
            .returning(|_, _| Ok(true));

        let ann = announcement.clone();
        service
            .announcement_repo
            .expect_create()
            .withf(move |_, announcement| announcement == &ann)
            .returning(|_, _| Err(AnnouncementDuplicate));

        let aid = announcement.id.clone();
        let ann = announcement.clone();
        service
            .announcement_repo
            .expect_find_by_id()
            .withf(move |_, announcement_id| announcement_id == &aid)
            .returning(move |_, _| Ok(ann.clone()));

        service.create(&announcement).await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "AnnouncementDuplicate")]
    async fn duplicate_error_case() {
        let conn = get_test_db_conn().await.unwrap();

        let mut service = MockRepositoryManager::new(conn);
        let announcement: Announcement = Faker.fake();

        let cid = announcement.course_id.clone();
        service
            .course_repo
            .expect_exist_by_id()
            .withf(move |_, course_id| course_id == &cid.clone())
            .returning(|_, _| Ok(true));

        let ann = announcement.clone();
        service
            .announcement_repo
            .expect_create()
            .withf(move |_, announcement| announcement == &ann)
            .returning(|_, _| Err(AnnouncementDuplicate));

        let aid = announcement.id.clone();
        let ann: Announcement = Faker.fake();
        service
            .announcement_repo
            .expect_find_by_id()
            .withf(move |_, announcement_id| announcement_id == &aid)
            .returning(move |_, _| Ok(ann.clone()));

        service.create(&announcement).await.unwrap();
    }
    #[tokio::test]
    async fn success_case() {
        let conn = get_test_db_conn().await.unwrap();

        let mut service = MockRepositoryManager::new(conn);
        let announcement: Announcement = Faker.fake();

        let cid = announcement.course_id.clone();
        service
            .course_repo
            .expect_exist_by_id()
            .withf(move |_, course_id| course_id == &cid.clone())
            .returning(|_, _| Ok(true));

        let ann = announcement.clone();
        service
            .announcement_repo
            .expect_create()
            .withf(move |_, announcement| announcement == &ann)
            .returning(|_, _| Ok(()));

        let cid = announcement.course_id.clone();
        let user: User = Faker.fake();
        let student = user.clone();
        service
            .registration_repo
            .expect_find_users_by_course_id()
            .withf(move |_, course_id| course_id == &cid)
            .returning(move |_, _| Ok(vec![student.clone()]));

        let aid = announcement.id.clone();
        let uid = user.id.clone();
        service
            .unread_announcement_repo
            .expect_create()
            .withf(move |_, announcement_id, user_id| announcement_id == &aid && user_id == &uid)
            .returning(|_, _, _| Ok(()));

        service.create(&announcement).await.unwrap();
    }
}
