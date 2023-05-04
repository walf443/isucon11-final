use crate::repos::class_repository::ClassRepositoryInfra;
use crate::repos::course_repository::CourseRepositoryInfra;
use crate::repos::registration_repository::RegistrationRepositoryInfra;
use crate::repos::submission_repository::SubmissionRepositoryInfra;
use crate::storages::submission_file_storage::SubmissionFileStorageInfra;
use isucholar_core::db::DBPool;
use isucholar_core::repos::class_repository::HaveClassRepository;
use isucholar_core::repos::course_repository::HaveCourseRepository;
use isucholar_core::repos::registration_repository::HaveRegistrationRepository;
use isucholar_core::repos::submission_repository::HaveSubmissionRepository;
use isucholar_core::services::submission_service::SubmissionServiceImpl;
use isucholar_core::services::HaveDBPool;
use isucholar_core::storages::submission_file_storage::HaveSubmissionFileStorage;
use std::sync::Arc;

#[derive(Clone)]
pub struct SubmissionServiceInfra {
    pool: Arc<DBPool>,
    class_repo: ClassRepositoryInfra,
    course_repo: CourseRepositoryInfra,
    submission_repo: SubmissionRepositoryInfra,
    submission_file_storage: SubmissionFileStorageInfra,
    registration_repo: RegistrationRepositoryInfra,
}

impl SubmissionServiceInfra {
    pub fn new(pool: Arc<DBPool>) -> Self {
        Self {
            pool,
            class_repo: ClassRepositoryInfra {},
            course_repo: CourseRepositoryInfra {},
            submission_repo: SubmissionRepositoryInfra {},
            submission_file_storage: SubmissionFileStorageInfra::new(),
            registration_repo: RegistrationRepositoryInfra {},
        }
    }
}

impl SubmissionServiceImpl for SubmissionServiceInfra {}

impl HaveDBPool for SubmissionServiceInfra {
    fn get_db_pool(&self) -> &DBPool {
        &self.pool
    }
}

impl HaveClassRepository for SubmissionServiceInfra {
    type Repo = ClassRepositoryInfra;

    fn class_repo(&self) -> &Self::Repo {
        &self.class_repo
    }
}

impl HaveSubmissionRepository for SubmissionServiceInfra {
    type Repo = SubmissionRepositoryInfra;

    fn submission_repo(&self) -> &Self::Repo {
        &self.submission_repo
    }
}

impl HaveCourseRepository for SubmissionServiceInfra {
    type Repo = CourseRepositoryInfra;

    fn course_repo(&self) -> &Self::Repo {
        &self.course_repo
    }
}

impl HaveRegistrationRepository for SubmissionServiceInfra {
    type Repo = RegistrationRepositoryInfra;

    fn registration_repo(&self) -> &Self::Repo {
        &self.registration_repo
    }
}

impl HaveSubmissionFileStorage for SubmissionServiceInfra {
    type Storage = SubmissionFileStorageInfra;

    fn submission_file_storage(&self) -> &Self::Storage {
        &self.submission_file_storage
    }
}
