use crate::db::{DBConn, TxConn};
use crate::models::submission::{CreateSubmission, Submission};
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SubmissionRepository {
    async fn create_or_update<'c>(
        &self,
        tx: &mut TxConn<'c>,
        submission: &CreateSubmission,
    ) -> Result<()>;
    async fn count_by_class_id(&self, conn: &mut DBConn, class_id: &str) -> Result<i64>;
    async fn update_score_by_user_code_and_class_id<'c>(
        &self,
        tx: &mut TxConn,
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
    async fn find_all_by_class_id_in_tx<'c>(
        &self,
        tx: &mut TxConn,
        class_id: &str,
    ) -> Result<Vec<Submission>>;
}

pub trait HaveSubmissionRepository {
    type Repo: Sync + SubmissionRepository;

    fn submission_repo(&self) -> &Self::Repo;
}
