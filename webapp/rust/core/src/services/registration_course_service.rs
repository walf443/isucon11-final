use crate::models::course::Course;
use crate::repos::registration_course_repository::{
    HaveRegistrationCourseRepository, RegistrationCourseRepository,
};
use crate::services::error::Result;
use crate::services::HaveDBPool;
use async_trait::async_trait;

#[async_trait]
pub trait RegistrationCourseService {
    async fn find_courses_by_user_id(&self, user_id: &str) -> Result<Vec<Course>>;
}

pub trait HaveRegistrationCourseService {
    type Service: RegistrationCourseService;

    fn registration_course_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait RegistrationCourseServiceImpl:
    Sync + HaveDBPool + HaveRegistrationCourseRepository
{
    async fn find_courses_by_user_id(&self, user_id: &str) -> Result<Vec<Course>> {
        let pool = self.get_db_pool();
        let result = self
            .registration_course_repo()
            .find_courses_by_user_id(pool, user_id)
            .await?;

        Ok(result)
    }
}

#[async_trait]
impl<S: RegistrationCourseServiceImpl> RegistrationCourseService for S {
    async fn find_courses_by_user_id(&self, user_id: &str) -> Result<Vec<Course>> {
        RegistrationCourseServiceImpl::find_courses_by_user_id(self, user_id).await
    }
}
