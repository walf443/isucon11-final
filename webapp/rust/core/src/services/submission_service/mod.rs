#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait SubmissionService {
    async fn update_user_scores_by_class_id(
        &self,
        class_id: &ClassID,
        scores: &Vec<Score>,
    ) -> Result<()>;
}

use crate::models::class::ClassID;
use crate::models::score::Score;
use crate::models::user::UserCode;
use crate::repos::class_repository::{ClassRepository, HaveClassRepository};
use crate::repos::submission_repository::{HaveSubmissionRepository, SubmissionRepository};
use crate::services::error::Error::{ClassIsNotSubmissionClosed, ClassNotFound};
use crate::services::error::Result;
use crate::services::HaveDBPool;
use async_trait::async_trait;

pub trait HaveSubmissionService {
    type Service: SubmissionService;
    fn submission_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait SubmissionServiceImpl:
    Sync + HaveDBPool + HaveClassRepository + HaveSubmissionRepository
{
    async fn update_user_scores_by_class_id(
        &self,
        class_id: &ClassID,
        scores: &Vec<Score>,
    ) -> Result<()> {
        let pool = self.get_db_pool();
        let mut tx = pool.begin().await?;

        let submission_closed = self
            .class_repo()
            .find_submission_closed_by_id_with_shared_lock(&mut tx, &class_id)
            .await?;

        if let Some(submission_closed) = submission_closed {
            if !submission_closed {
                return Err(ClassIsNotSubmissionClosed);
            }
        } else {
            return Err(ClassNotFound);
        }

        let submission_repo = self.submission_repo();

        for score in scores {
            let user_code = UserCode::new(score.user_code.clone());
            submission_repo
                .update_score_by_user_code_and_class_id(&mut tx, &user_code, &class_id, score.score)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl<S: SubmissionServiceImpl> SubmissionService for S {
    async fn update_user_scores_by_class_id(
        &self,
        class_id: &ClassID,
        scores: &Vec<Score>,
    ) -> Result<()> {
        SubmissionServiceImpl::update_user_scores_by_class_id(self, class_id, scores).await
    }
}
