#[cfg(test)]
mod tests {
    use crate::db::get_test_db_conn;
    use crate::models::course::CourseID;
    use crate::models::course_status::CourseStatus;
    use crate::repos::manager::tests::MockRepositoryManager;
    use crate::services::course_service::CourseServiceImpl;
    use fake::{Fake, Faker};

    #[tokio::test]
    #[should_panic(expected = "CourseNotFound")]
    async fn record_not_exist_case() {
        let conn = get_test_db_conn().await.unwrap();
        let mut service = MockRepositoryManager::new(conn);

        let course_id: CourseID = Faker.fake();
        let status: CourseStatus = Faker.fake();

        let cid = course_id.clone();
        service
            .course_repo
            .expect_for_update_by_id()
            .withf(move |_, course_id| course_id == &cid)
            .returning(|_, _| Ok(false));

        service
            .update_status_by_id(&course_id, &status)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn success_case() {
        let conn = get_test_db_conn().await.unwrap();
        let mut service = MockRepositoryManager::new(conn);

        let course_id: CourseID = Faker.fake();
        let status: CourseStatus = Faker.fake();

        let cid = course_id.clone();
        service
            .course_repo
            .expect_for_update_by_id()
            .withf(move |_, course_id| course_id == &cid)
            .returning(|_, _| Ok(true));

        let cid = course_id.clone();
        let st = status.clone();
        service
            .course_repo
            .expect_update_status_by_id()
            .withf(move |_, course_id, status| course_id == &cid && status == &st)
            .returning(|_, _, _| Ok(()));

        service
            .update_status_by_id(&course_id, &status)
            .await
            .unwrap();
    }
}
