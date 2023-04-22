use crate::repos::user_repository::UserRepositoryImpl;
use isucholar_core::db::DBPool;
use isucholar_core::repos::user_repository::HaveUserRepository;
use isucholar_core::services::grade_summary_service::GradeSummaryServiceImpl;
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct GradeSummaryServiceInfra {
    db_pool: Arc<DBPool>,
    user_repo: UserRepositoryImpl,
}

impl GradeSummaryServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            user_repo: UserRepositoryImpl {},
        }
    }
}

impl GradeSummaryServiceImpl for GradeSummaryServiceInfra {}

impl HaveDBPool for GradeSummaryServiceInfra {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}

impl HaveUserRepository for GradeSummaryServiceInfra {
    type Repo = UserRepositoryImpl;

    fn user_repo(&self) -> &Self::Repo {
        &self.user_repo
    }
}