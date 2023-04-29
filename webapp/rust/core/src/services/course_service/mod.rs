use crate::models::course::{Course, CourseWithTeacher, CreateCourse};
use crate::models::user::User;
use crate::repos::course_repository::{CourseRepository, HaveCourseRepository, SearchCoursesQuery};
use crate::repos::registration_course_repository::{
    HaveRegistrationCourseRepository, RegistrationCourseRepository,
};
use crate::repos::transaction_repository::{HaveTransactionRepository, TransactionRepository};
use crate::repos::user_repository::{HaveUserRepository, UserRepository};
use crate::services::error::Result;
use crate::services::HaveDBPool;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait CourseService: Sync {
    async fn create(&self, course: &CreateCourse) -> Result<String>;
    async fn find_all_with_teacher(
        &self,
        limit: i64,
        offset: i64,
        query: &SearchCoursesQuery,
    ) -> Result<Vec<CourseWithTeacher>>;
    async fn find_open_courses_by_user_id(&self, user_id: &str) -> Result<Vec<(Course, User)>>;
}

pub trait HaveCourseService {
    type Service: CourseService;
    fn course_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait CourseServiceImpl:
    Sync
    + HaveDBPool
    + HaveTransactionRepository
    + HaveUserRepository
    + HaveCourseRepository
    + HaveRegistrationCourseRepository
{
    async fn create(&self, course: &CreateCourse) -> Result<String> {
        let db_pool = self.get_db_pool();

        let course_id = self.course_repo().create(&db_pool, course).await?;

        Ok(course_id)
    }

    async fn find_all_with_teacher(
        &self,
        limit: i64,
        offset: i64,
        query: &SearchCoursesQuery,
    ) -> Result<Vec<CourseWithTeacher>> {
        let db_pool = self.get_db_pool();

        let courses = self
            .course_repo()
            .find_all_with_teacher(&db_pool, limit, offset, query)
            .await?;
        Ok(courses)
    }

    async fn find_open_courses_by_user_id(&self, user_id: &str) -> Result<Vec<(Course, User)>> {
        let db_pool = self.get_db_pool();
        let mut tx = self.transaction_repo().begin(db_pool).await?;

        let courses = self
            .registration_course_repo()
            .find_open_courses_by_user_id(&mut tx, &user_id)
            .await?;

        let mut results: Vec<(Course, User)> = Vec::with_capacity(courses.len());

        for course in courses {
            let teacher = self
                .user_repo()
                .find_in_tx(&mut tx, &course.teacher_id)
                .await?;

            results.push((course, teacher))
        }

        tx.commit().await?;

        Ok(results)
    }
}

#[async_trait]
impl<S: CourseServiceImpl> CourseService for S {
    async fn create(&self, course: &CreateCourse) -> Result<String> {
        CourseServiceImpl::create(self, course).await
    }

    async fn find_all_with_teacher(
        &self,
        limit: i64,
        offset: i64,
        query: &SearchCoursesQuery,
    ) -> Result<Vec<CourseWithTeacher>> {
        CourseServiceImpl::find_all_with_teacher(self, limit, offset, query).await
    }

    async fn find_open_courses_by_user_id(&self, user_id: &str) -> Result<Vec<(Course, User)>> {
        CourseServiceImpl::find_open_courses_by_user_id(self, user_id).await
    }
}
