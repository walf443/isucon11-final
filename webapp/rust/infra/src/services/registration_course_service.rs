use crate::repos::course_repository::CourseRepositoryInfra;
use crate::repos::registration_course_repository::RegistrationCourseRepositoryInfra;
use crate::repos::registration_repository::RegistrationRepositoryInfra;
use isucholar_core::db::DBPool;
use isucholar_core::repos::course_repository::HaveCourseRepository;
use isucholar_core::repos::registration_course_repository::HaveRegistrationCourseRepository;
use isucholar_core::repos::registration_repository::HaveRegistrationRepository;
use isucholar_core::repos::transaction_repository::{
    HaveTransactionRepository, TransactionRepositoryImpl,
};
use isucholar_core::services::registration_course_service::RegistrationCourseServiceImpl;
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct RegistrationCourseServiceInfra {
    db_pool: Arc<DBPool>,
    registration_course_repo: RegistrationCourseRepositoryInfra,
    transaction_repo: TransactionRepositoryImpl,
    course_repo: CourseRepositoryInfra,
    registration_repo: RegistrationRepositoryInfra,
}

impl RegistrationCourseServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            registration_course_repo: RegistrationCourseRepositoryInfra {},
            transaction_repo: TransactionRepositoryImpl {},
            course_repo: CourseRepositoryInfra {},
            registration_repo: RegistrationRepositoryInfra {},
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
    type Repo = RegistrationCourseRepositoryInfra;

    fn registration_course_repo(&self) -> &Self::Repo {
        &self.registration_course_repo
    }
}

impl HaveTransactionRepository for RegistrationCourseServiceInfra {
    type Repo = TransactionRepositoryImpl;

    fn transaction_repo(&self) -> &Self::Repo {
        &self.transaction_repo
    }
}

impl HaveRegistrationRepository for RegistrationCourseServiceInfra {
    type Repo = RegistrationRepositoryInfra;

    fn registration_repo(&self) -> &Self::Repo {
        &self.registration_repo
    }
}

impl HaveCourseRepository for RegistrationCourseServiceInfra {
    type Repo = CourseRepositoryInfra;

    fn course_repo(&self) -> &Self::Repo {
        &self.course_repo
    }
}
