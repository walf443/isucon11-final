use crate::models::course::{Course, CourseID};
use crate::models::course_status::CourseStatus;
use crate::models::user::UserID;
use crate::repos::course_repository::{CourseRepository, HaveCourseRepository};
use crate::repos::registration_course_repository::{
    HaveRegistrationCourseRepository, RegistrationCourseRepository,
};
use crate::repos::registration_repository::{HaveRegistrationRepository, RegistrationRepository};
use crate::services::error::{Error, RegistrationCourseValidationError, Result};
use crate::services::HaveDBPool;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait RegistrationCourseService {
    async fn find_courses_by_user_id(&self, user_id: &UserID) -> Result<Vec<Course>>;
    async fn create(&self, user_id: &UserID, course_ids: &[CourseID]) -> Result<()>;
}

pub trait HaveRegistrationCourseService {
    type Service: RegistrationCourseService;

    fn registration_course_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait RegistrationCourseServiceImpl:
    Sync
    + HaveDBPool
    + HaveRegistrationCourseRepository
    + HaveRegistrationRepository
    + HaveCourseRepository
{
    async fn find_courses_by_user_id(&self, user_id: &UserID) -> Result<Vec<Course>> {
        let pool = self.get_db_pool();
        let mut conn = pool.acquire().await?;
        let result = self
            .registration_course_repo()
            .find_courses_by_user_id(&mut conn, user_id)
            .await?;

        Ok(result)
    }

    async fn create(&self, user_id: &UserID, course_ids: &[CourseID]) -> Result<()> {
        let pool = self.get_db_pool();
        let mut tx = pool.begin().await?;

        let course_repo = self.course_repo();
        let registration_course_repo = self.registration_course_repo();
        let registration_repo = self.registration_repo();

        let mut errors = RegistrationCourseValidationError::default();
        let mut newly_added = Vec::new();
        for course_id in course_ids {
            let course = course_repo
                .find_for_share_lock_by_id(&mut tx, course_id)
                .await?;
            if course.is_none() {
                errors.course_not_found.push(course_id.clone());
                continue;
            }
            let course = course.unwrap();

            if course.status != CourseStatus::Registration {
                errors.not_registrable_status.push(course.id);
                continue;
            }

            // すでに履修登録済みの科目は無視する
            let is_exist = registration_repo
                .exist_by_user_id_and_course_id(&mut tx, user_id, course_id)
                .await?;
            if is_exist {
                continue;
            }

            newly_added.push(course);
        }

        let already_registered = registration_course_repo
            .find_open_courses_by_user_id(&mut tx, user_id)
            .await?;

        for course1 in &newly_added {
            for course2 in already_registered.iter().chain(newly_added.iter()) {
                if course1.id != course2.id
                    && course1.period == course2.period
                    && course1.day_of_week == course2.day_of_week
                {
                    errors.schedule_conflict.push(course1.id.to_owned());
                    break;
                }
            }
        }

        if !errors.course_not_found.is_empty()
            || !errors.not_registrable_status.is_empty()
            || !errors.schedule_conflict.is_empty()
        {
            return Err(Error::RegistrationCourseValidationError(errors));
        }

        for course in newly_added {
            registration_repo
                .create_or_update(&mut tx, user_id, &course.id)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl<S: RegistrationCourseServiceImpl> RegistrationCourseService for S {
    async fn find_courses_by_user_id(&self, user_id: &UserID) -> Result<Vec<Course>> {
        RegistrationCourseServiceImpl::find_courses_by_user_id(self, user_id).await
    }

    async fn create(&self, user_id: &UserID, course_ids: &[CourseID]) -> Result<()> {
        RegistrationCourseServiceImpl::create(self, user_id, course_ids).await
    }
}
