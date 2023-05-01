use crate::repos::course_repository::CourseRepositoryInfra;
use crate::repos::registration_course_repository::RegistrationCourseRepositoryInfra;
use crate::repos::user_repository::UserRepositoryInfra;
use isucholar_core::db::DBPool;
use isucholar_core::repos::course_repository::HaveCourseRepository;
use isucholar_core::repos::registration_course_repository::HaveRegistrationCourseRepository;
use isucholar_core::repos::user_repository::HaveUserRepository;
use isucholar_core::services::course_service::CourseServiceImpl;
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

pub mod announcement_service;
pub mod class_service;
pub mod course_service;
pub mod grade_summary_service;
pub mod manager;
pub mod registration_course_service;
pub mod unread_announcement_service;
pub mod user_service;

#[derive(Clone)]
pub struct CourseServiceInfra {
    db_pool: Arc<DBPool>,
    user_repo: UserRepositoryInfra,
    course_repo: CourseRepositoryInfra,
    registration_course_repo: RegistrationCourseRepositoryInfra,
}

impl CourseServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            user_repo: UserRepositoryInfra {},
            course_repo: CourseRepositoryInfra {},
            registration_course_repo: RegistrationCourseRepositoryInfra {},
        }
    }
}

impl CourseServiceImpl for CourseServiceInfra {}

impl HaveDBPool for CourseServiceInfra {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}

impl HaveUserRepository for CourseServiceInfra {
    type Repo = UserRepositoryInfra;

    fn user_repo(&self) -> &Self::Repo {
        &self.user_repo
    }
}

impl HaveRegistrationCourseRepository for CourseServiceInfra {
    type Repo = RegistrationCourseRepositoryInfra;

    fn registration_course_repo(&self) -> &Self::Repo {
        &self.registration_course_repo
    }
}
impl HaveCourseRepository for CourseServiceInfra {
    type Repo = CourseRepositoryInfra;

    fn course_repo(&self) -> &Self::Repo {
        &self.course_repo
    }
}
