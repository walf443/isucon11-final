use crate::models::summary::Summary;
use crate::repos::user_repository::{HaveUserRepository, UserRepository};
use crate::services::error::Result;
use crate::services::HaveDBPool;
use crate::util;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait GradeSummaryService {
    async fn get_summary_by_user_gpa(&self, user_gpa: f64, user_credit: i64) -> Result<Summary>;
}

#[async_trait]
pub trait HaveGradeSummaryService {
    type Service: GradeSummaryService;
    fn grade_summary_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait GradeSummaryServiceImpl: Sync + HaveDBPool + HaveUserRepository {
    async fn get_summary_by_user_gpa(&self, user_gpa: f64, user_credit: i64) -> Result<Summary> {
        let pool = self.get_db_pool();
        let gpas = self.user_repo().find_gpas_group_by_user_id(pool).await?;

        Ok(Summary {
            credits: user_credit,
            gpa: user_gpa,
            gpa_t_score: util::t_score_f64(user_gpa, &gpas),
            gpa_avg: util::average_f64(&gpas, 0.0),
            gpa_max: util::max_f64(&gpas, 0.0),
            gpa_min: util::min_f64(&gpas, 0.0),
        })
    }
}

#[async_trait]
impl<S: GradeSummaryServiceImpl> GradeSummaryService for S {
    async fn get_summary_by_user_gpa(&self, user_gpa: f64, user_credit: i64) -> Result<Summary> {
        GradeSummaryServiceImpl::get_summary_by_user_gpa(self, user_gpa, user_credit).await
    }
}
