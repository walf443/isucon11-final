use crate::repos::class_repository::ClassRepositoryInfra;
use crate::repos::registration_course_repository::RegistrationCourseRepositoryInfra;
use crate::repos::submission_repository::SubmissionRepositoryInfra;
use isucholar_core::db::DBPool;
use isucholar_core::repos::class_repository::HaveClassRepository;
use isucholar_core::repos::registration_course_repository::HaveRegistrationCourseRepository;
use isucholar_core::repos::submission_repository::HaveSubmissionRepository;
use isucholar_core::services::class_service::ClassServiceImpl;
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct ClassServiceInfra {
    db_pool: Arc<DBPool>,
    class_repo: ClassRepositoryInfra,
    submission_repo: SubmissionRepositoryInfra,
    registration_course_repo: RegistrationCourseRepositoryInfra,
}

impl ClassServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            class_repo: ClassRepositoryInfra {},
            submission_repo: SubmissionRepositoryInfra {},
            registration_course_repo: RegistrationCourseRepositoryInfra {},
        }
    }
}

impl ClassServiceImpl for ClassServiceInfra {}

impl HaveDBPool for ClassServiceInfra {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}

impl HaveClassRepository for ClassServiceInfra {
    type Repo = ClassRepositoryInfra;

    fn class_repo(&self) -> &Self::Repo {
        &self.class_repo
    }
}

impl HaveSubmissionRepository for ClassServiceInfra {
    type Repo = SubmissionRepositoryInfra;

    fn submission_repo(&self) -> &Self::Repo {
        &self.submission_repo
    }
}
impl HaveRegistrationCourseRepository for ClassServiceInfra {
    type Repo = RegistrationCourseRepositoryInfra;

    fn registration_course_repo(&self) -> &Self::Repo {
        &self.registration_course_repo
    }
}
