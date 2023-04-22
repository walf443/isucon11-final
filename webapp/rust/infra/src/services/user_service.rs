use crate::repos::user_repository::UserRepositoryInfra;
use isucholar_core::db::DBPool;
use isucholar_core::repos::user_repository::HaveUserRepository;
use isucholar_core::services::user_service::UserServiceImpl;
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserServiceInfra {
    db_pool: Arc<DBPool>,
    user_repo: UserRepositoryInfra,
}

impl UserServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            user_repo: UserRepositoryInfra {},
        }
    }
}

impl UserServiceImpl for UserServiceInfra {}

impl HaveDBPool for UserServiceInfra {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}

impl HaveUserRepository for UserServiceInfra {
    type Repo = UserRepositoryInfra;

    fn user_repo(&self) -> &Self::Repo {
        &self.user_repo
    }
}
