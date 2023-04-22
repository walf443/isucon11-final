use crate::repos::course_repository::CourseRepositoryImpl;
use crate::repos::registration_course_repository::RegistrationCourseRepositoryImpl;
use crate::repos::registration_repository::RegistrationRepositoryImpl;
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
    registration_course_repo: RegistrationCourseRepositoryImpl,
    transaction_repo: TransactionRepositoryImpl,
    course_repo: CourseRepositoryImpl,
    registration_repo: RegistrationRepositoryImpl,
}

impl RegistrationCourseServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            registration_course_repo: RegistrationCourseRepositoryImpl {},
            transaction_repo: TransactionRepositoryImpl {},
            course_repo: CourseRepositoryImpl {},
            registration_repo: RegistrationRepositoryImpl {},
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

impl HaveTransactionRepository for RegistrationCourseServiceInfra {
    type Repo = TransactionRepositoryImpl;

    fn transaction_repo(&self) -> &Self::Repo {
        &self.transaction_repo
    }
}

impl HaveRegistrationRepository for RegistrationCourseServiceInfra {
    type Repo = RegistrationRepositoryImpl;

    fn registration_repo(&self) -> &Self::Repo {
        &self.registration_repo
    }
}

impl HaveCourseRepository for RegistrationCourseServiceInfra {
    type Repo = CourseRepositoryImpl;

    fn course_repo(&self) -> &Self::Repo {
        &self.course_repo
    }
}
