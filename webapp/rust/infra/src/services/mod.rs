use crate::repos::registration_course_repository::RegistrationCourseRepositoryImpl;
use crate::repos::user_repository::UserRepositoryImpl;
use isucholar_core::db::DBPool;
use isucholar_core::repos::registration_course_repository::HaveRegistrationCourseRepository;
use isucholar_core::repos::transaction_repository::{
    HaveTransactionRepository, TransactionRepositoryImpl,
};
use isucholar_core::repos::user_repository::HaveUserRepository;
use isucholar_core::services::course_service::CourseServiceImpl;
use isucholar_core::services::HaveDBPool;
use std::sync::Arc;

pub mod announcement_service;
pub mod class_service;
pub mod course_service;
pub mod manager;
pub mod registration_course_service;
pub mod unread_announcement_service;
pub mod user_service;

#[derive(Clone)]
pub struct CourseServiceInfra {
    db_pool: Arc<DBPool>,
    transaction_repo: TransactionRepositoryImpl,
    user_repo: UserRepositoryImpl,
    registration_course_repo: RegistrationCourseRepositoryImpl,
}

impl CourseServiceInfra {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self {
            db_pool,
            transaction_repo: TransactionRepositoryImpl {},
            user_repo: UserRepositoryImpl {},
            registration_course_repo: RegistrationCourseRepositoryImpl {},
        }
    }
}

impl CourseServiceImpl for CourseServiceInfra {}

impl HaveDBPool for CourseServiceInfra {
    fn get_db_pool(&self) -> &DBPool {
        &self.db_pool
    }
}

impl HaveTransactionRepository for CourseServiceInfra {
    type Repo = TransactionRepositoryImpl;

    fn transaction_repo(&self) -> &Self::Repo {
        &self.transaction_repo
    }
}

impl HaveUserRepository for CourseServiceInfra {
    type Repo = UserRepositoryImpl;

    fn user_repo(&self) -> &Self::Repo {
        &self.user_repo
    }
}

impl HaveRegistrationCourseRepository for CourseServiceInfra {
    type Repo = RegistrationCourseRepositoryImpl;

    fn registration_course_repo(&self) -> &Self::Repo {
        &self.registration_course_repo
    }
}
