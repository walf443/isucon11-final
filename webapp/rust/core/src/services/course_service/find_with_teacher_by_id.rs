#[cfg(test)]
mod tests {
    use crate::db::get_test_db_conn;
    use crate::models::course::{CourseID, CourseWithTeacher};
    use crate::repos::manager::tests::MockRepositoryManager;
    use crate::services::course_service::CourseService;
    use fake::{Fake, Faker};

    #[tokio::test]
    async fn none_case() {
        let conn = get_test_db_conn().await.unwrap();
        let mut service = MockRepositoryManager::new(conn);

        service
            .course_repo
            .expect_find_with_teacher_by_id()
            .returning(|_, _| Ok(None));

        let course_id: CourseID = Faker.fake();
        let result = service.find_with_teacher_by_id(&course_id).await.unwrap();

        assert_eq!(result.is_none(), true);
    }

    #[tokio::test]
    async fn some_case() {
        let conn = get_test_db_conn().await.unwrap();
        let mut service = MockRepositoryManager::new(conn);

        let course: CourseWithTeacher = Faker.fake();

        let c = course.clone();
        service
            .course_repo
            .expect_find_with_teacher_by_id()
            .returning(move |_, _| Ok(Some(c.clone())));

        let course_id: CourseID = Faker.fake();
        let result = service.find_with_teacher_by_id(&course_id).await.unwrap();

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), course);
    }
}
