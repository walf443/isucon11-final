#[cfg(test)]
mod tests {
    use crate::db::get_test_db_conn;
    use crate::repos::course_repository::SearchCoursesQuery;
    use crate::repos::manager::tests::MockRepositoryManager;
    use crate::services::course_service::CourseServiceImpl;
    use fake::{Fake, Faker};

    #[tokio::test]
    async fn success_case() {
        let conn = get_test_db_conn().await.unwrap();
        let mut service = MockRepositoryManager::new(conn);

        let limit = 10;
        let offset = 20;
        let query: SearchCoursesQuery = Faker.fake();
        let q = query.clone();

        service
            .course_repo
            .expect_find_all_with_teacher()
            .withf(move |_, l, o, query| l == &limit && o == &offset && query == &q)
            .returning(|_, _, _, _| Ok(vec![Faker.fake()]));

        let result = service.find_all_with_teacher(10, 20, &query).await.unwrap();

        assert_eq!(result.len(), 1);
    }
}
