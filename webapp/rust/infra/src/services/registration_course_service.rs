use crate::repos::registration_course_repository::RegistrationCourseRepositoryImpl;
use isucholar_core::db::DBPool;
use isucholar_core::repos::registration_course_repository::HaveRegistrationCourseRepository;
use isucholar_core::services::registration_course_service::RegistrationCourseServiceImpl;
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct RegistrationCourseServiceInfra {
    db_pool: Arc<DBPool>,
    registration_course_repo: RegistrationCourseRepositoryImpl,
}

impl RegistrationCourseServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            registration_course_repo: RegistrationCourseRepositoryImpl {},
        }
    }
}

impl RegistrationCourseServiceImpl for RegistrationCourseServiceInfra {}

impl HaveDBPool for RegistrationCourseServiceInfra {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}

impl HaveRegistrationCourseRepository for RegistrationCourseServiceInfra {
    type Repo = RegistrationCourseRepositoryImpl;

    fn registration_course_repo(&self) -> &Self::Repo {
        &self.registration_course_repo
    }
}
