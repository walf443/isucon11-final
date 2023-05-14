#[cfg(test)]
mod tests {
    use crate::db::get_test_db_conn;
    use crate::models::course::{Course, CreateCourse};
    use crate::repos::error::ReposError::{CourseDuplicate, TestError};
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
    #[should_panic(expected = "ReposError(CourseDuplicate)")]
    async fn duplicate_error_case() {
        let conn = get_test_db_conn().await.unwrap();
        let mut service = MockRepositoryManager::new(conn);

        let course: CreateCourse = Faker.fake();

        service
            .course_repo
            .expect_create()
            .returning(move |_, _| Err(CourseDuplicate));

        let c: Course = Faker.fake();
        service
            .course_repo
            .expect_find_by_code()
            .returning(move |_, _| Ok(c.clone()));

        service.create(&course).await.unwrap();
    }

    #[tokio::test]
    async fn duplicate_success_case() {
        let conn = get_test_db_conn().await.unwrap();
        let mut service = MockRepositoryManager::new(conn);

        let course: CreateCourse = Faker.fake();

        service
            .course_repo
            .expect_create()
            .returning(move |_, _| Err(CourseDuplicate));

        let c: Course = Course {
            id: course.id.clone(),
            code: course.code.clone(),
            type_: course.type_.clone(),
            name: course.name.clone(),
            description: course.description.clone(),
            credit: course.credit.clone() as u8,
            period: course.period.clone() as u8,
            day_of_week: course.day_of_week.clone(),
            teacher_id: Faker.fake(),
            keywords: course.keywords.clone(),
            status: Faker.fake(),
        };
        let ccode = c.code.clone();
        service
            .course_repo
            .expect_find_by_code()
            .withf(move |_, code| code == &ccode)
            .returning(move |_, _| Ok(c.clone()));

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
