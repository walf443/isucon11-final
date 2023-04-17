use crate::models::course::Course;
use crate::models::user::User;
use crate::repos::registration_course_repository::{
    HaveRegistrationCourseRepository, RegistrationCourseRepository,
};
use crate::repos::transaction_repository::{HaveTransactionRepository, TransactionRepository};
use crate::repos::user_repository::{HaveUserRepository, UserRepository};
use crate::services::error::Result;
use crate::services::HaveDBPool;
use async_trait::async_trait;

#[async_trait]
pub trait CourseService: Sync {
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
    + HaveRegistrationCourseRepository
{
    async fn find_open_courses_by_user_id(&self, user_id: &str) -> Result<Vec<(Course, User)>> {
        let db_pool = self.get_db_pool();
        let mut tx = self.transaction_repository().begin(db_pool).await?;

        let courses = self
            .registration_course_repo()
            .find_open_courses_by_user_id_in_tx(&mut tx, &user_id)
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
    async fn find_open_courses_by_user_id(&self, user_id: &str) -> Result<Vec<(Course, User)>> {
        CourseServiceImpl::find_open_courses_by_user_id(self, user_id).await
    }
}
