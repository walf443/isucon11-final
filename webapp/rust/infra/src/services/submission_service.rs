use crate::repos::class_repository::ClassRepositoryInfra;
use crate::repos::submission_repository::SubmissionRepositoryInfra;
use isucholar_core::db::DBPool;
use isucholar_core::repos::class_repository::HaveClassRepository;
use isucholar_core::repos::submission_repository::HaveSubmissionRepository;
use isucholar_core::services::submission_service::{SubmissionService, SubmissionServiceImpl};
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct SubmissionServiceInfra {
    pool: Arc<DBPool>,
    class_repo: ClassRepositoryInfra,
    submission_repo: SubmissionRepositoryInfra,
}

impl SubmissionServiceInfra {
    pub fn new(pool: Arc<DBPool>) -> Self {
        Self {
            pool,
            class_repo: ClassRepositoryInfra {},
            submission_repo: SubmissionRepositoryInfra {},
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
