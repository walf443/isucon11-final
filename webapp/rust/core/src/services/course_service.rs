use crate::models::course::{Course, CourseID, CourseWithTeacher, CreateCourse};
use crate::models::course_status::CourseStatus;
use crate::models::user::{User, UserID};
use crate::repos::course_repository::{CourseRepository, HaveCourseRepository, SearchCoursesQuery};
use crate::repos::error::ReposError;
use crate::repos::registration_course_repository::{
    HaveRegistrationCourseRepository, RegistrationCourseRepository,
};
use crate::repos::user_repository::{HaveUserRepository, UserRepository};
use crate::services::error::Error::CourseNotFound;
use crate::services::error::Result;
use crate::services::HaveDBPool;
use async_trait::async_trait;

mod create;
mod find_all_with_teacher;
mod find_with_teacher_by_id;
mod update_status_by_id;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait CourseService: Sync {
    async fn create(&self, course: &CreateCourse) -> Result<CourseID>;
    async fn update_status_by_id(&self, course_id: &CourseID, status: &CourseStatus) -> Result<()>;
    async fn find_all_with_teacher(
        &self,
        limit: i64,
        offset: i64,
        query: &SearchCoursesQuery,
    ) -> Result<Vec<CourseWithTeacher>>;
    async fn find_with_teacher_by_id(
        &self,
        course_id: &CourseID,
    ) -> Result<Option<CourseWithTeacher>>;
    async fn find_open_courses_by_user_id(&self, user_id: &UserID) -> Result<Vec<(Course, User)>>;
}

pub trait HaveCourseService {
    type Service: CourseService;
    fn course_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait CourseServiceImpl:
    Sync + HaveDBPool + HaveUserRepository + HaveCourseRepository + HaveRegistrationCourseRepository
{
    async fn create(&self, req: &CreateCourse) -> Result<CourseID> {
        let db_pool = self.get_db_pool();
        let mut conn = db_pool.acquire().await?;

        let course_repo = self.course_repo();
        let result = course_repo.create(&mut conn, req).await;
        match result {
            Ok(course_id) => Ok(course_id),
            Err(e) => match e {
                ReposError::CourseDuplicate => {
                    let course = course_repo.find_by_code(&mut conn, &req.code).await?;

                    if req.type_ != course.type_
                        || req.name != course.name
                        || req.description != course.description
                        || req.credit != course.credit
                        || req.period != course.period
                        || req.day_of_week != course.day_of_week
                        || req.keywords != course.keywords
                    {
                        return Err(e.into());
                    } else {
                        return Ok(course.id);
                    }
                }
                _ => Err(e.into()),
            },
        }
    }

    async fn update_status_by_id(&self, course_id: &CourseID, status: &CourseStatus) -> Result<()> {
        let db_pool = self.get_db_pool();
        let course_repo = self.course_repo();
        let mut tx = db_pool.begin().await?;

        let is_exist = course_repo.for_update_by_id(&mut tx, course_id).await?;
        if !is_exist {
            return Err(CourseNotFound);
        }

        course_repo
            .update_status_by_id(&mut tx, course_id, status)
            .await?;

        tx.commit().await?;

        Ok(())
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
            .find_all_with_teacher(db_pool, limit, offset, query)
            .await?;
        Ok(courses)
    }

    async fn find_open_courses_by_user_id(&self, user_id: &UserID) -> Result<Vec<(Course, User)>> {
        let db_pool = self.get_db_pool();
        let mut tx = db_pool.begin().await?;

        let courses = self
            .registration_course_repo()
            .find_open_courses_by_user_id(&mut tx, user_id)
            .await?;

        let mut results: Vec<(Course, User)> = Vec::with_capacity(courses.len());

        for course in courses {
            let teacher = self.user_repo().find(&mut tx, &course.teacher_id).await?;

            results.push((course, teacher))
        }

        tx.commit().await?;

        Ok(results)
    }

    async fn find_with_teacher_by_id(
        &self,
        course_id: &CourseID,
    ) -> Result<Option<CourseWithTeacher>> {
        let pool = self.get_db_pool();
        let mut conn = pool.acquire().await?;
        let course = self
            .course_repo()
            .find_with_teacher_by_id(&mut conn, course_id)
            .await?;

        Ok(course)
    }
}

#[async_trait]
impl<S: CourseServiceImpl> CourseService for S {
    async fn create(&self, course: &CreateCourse) -> Result<CourseID> {
        CourseServiceImpl::create(self, course).await
    }

    async fn update_status_by_id(&self, course_id: &CourseID, status: &CourseStatus) -> Result<()> {
        CourseServiceImpl::update_status_by_id(self, course_id, status).await
    }

    async fn find_all_with_teacher(
        &self,
        limit: i64,
        offset: i64,
        query: &SearchCoursesQuery,
    ) -> Result<Vec<CourseWithTeacher>> {
        CourseServiceImpl::find_all_with_teacher(self, limit, offset, query).await
    }

    async fn find_with_teacher_by_id(
        &self,
        course_id: &CourseID,
    ) -> Result<Option<CourseWithTeacher>> {
        CourseServiceImpl::find_with_teacher_by_id(self, course_id).await
    }

    async fn find_open_courses_by_user_id(&self, user_id: &UserID) -> Result<Vec<(Course, User)>> {
        CourseServiceImpl::find_open_courses_by_user_id(self, user_id).await
    }
}
