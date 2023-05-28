use crate::models::class::ClassID;
use crate::models::submission::SubmissionWithUserCode;
use crate::models::user::UserID;
use crate::storages::StorageResult;
use async_trait::async_trait;

#[async_trait]
pub trait SubmissionFileStorage {
    async fn upload<B: bytes::Buf + Send>(
        &self,
        class_id: &ClassID,
        user_id: &UserID,
        data: &mut B,
    ) -> StorageResult<String>;

    async fn create_submissions_zip(
        &self,
        class_id: &ClassID,
        submissions: &[SubmissionWithUserCode],
    ) -> StorageResult<String>;
}

pub trait HaveSubmissionFileStorage {
    type Storage: Sync + SubmissionFileStorage;

    fn submission_file_storage(&self) -> &Self::Storage;
}
