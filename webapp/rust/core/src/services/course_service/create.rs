#[cfg(test)]
mod tests {
    use crate::db::get_test_db_conn;
    use crate::models::course::CreateCourse;
    use crate::repos::error::ReposError::TestError;
    use crate::repos::manager::tests::MockRepositoryManager;
    use crate::services::course_service::CourseServiceImpl;
    use fake::{Fake, Faker};

    #[tokio::test]
    async fn success_case() {
        let conn = get_test_db_conn().await.unwrap();
        let mut service = MockRepositoryManager::new(conn);

        let course: CreateCourse = Faker.fake();

        let course_id = course.id.clone();
        service
            .course_repo
            .expect_create()
            .returning(move |_, _| Ok(course_id.clone()));

        let got = service.create(&course).await.unwrap();
        assert_eq!(got, course.id)
    }

    #[tokio::test]
    #[should_panic(expected = "ReposError(TestError)")]
    async fn error_case() {
        let conn = get_test_db_conn().await.unwrap();
        let mut service = MockRepositoryManager::new(conn);

        let course: CreateCourse = Faker.fake();

        service
            .course_repo
            .expect_create()
            .returning(move |_, _| Err(TestError));

        service.create(&course).await.unwrap();
    }
}
