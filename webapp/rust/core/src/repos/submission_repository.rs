use crate::db::DBConn;
use crate::models::submission::{CreateSubmission, SubmissionWithUserCode};
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SubmissionRepository {
    async fn create_or_update(
        &self,
        conn: &mut DBConn,
        submission: &CreateSubmission,
    ) -> Result<()>;
    async fn count_by_class_id(&self, conn: &mut DBConn, class_id: &str) -> Result<i64>;
    async fn update_score_by_user_code_and_class_id(
        &self,
        conn: &mut DBConn,
        user_code: &str,
        class_id: &str,
        score: i64,
    ) -> Result<()>;
    async fn find_score_by_class_id_and_user_id(
        &self,
        conn: &mut DBConn,
        class_id: &str,
        user_id: &str,
    ) -> Result<Option<u8>>;
    async fn find_all_with_user_code_by_class_id(
        &self,
        conn: &mut DBConn,
        class_id: &str,
    ) -> Result<Vec<SubmissionWithUserCode>>;
}

pub trait HaveSubmissionRepository {
    type Repo: Sync + SubmissionRepository;

    fn submission_repo(&self) -> &Self::Repo;
}
