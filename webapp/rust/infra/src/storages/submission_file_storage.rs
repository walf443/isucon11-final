use async_trait::async_trait;
use isucholar_core::models::class::ClassID;
use isucholar_core::models::submission::SubmissionWithUserCode;
use isucholar_core::models::user::UserID;
use isucholar_core::storages::submission_file_storage::SubmissionFileStorage;
use isucholar_core::storages::StorageResult;
use isucholar_infra_storage_file::submission_file_storage::SubmissionFileStorageFile;

#[derive(Clone, Default)]
pub struct SubmissionFileStorageInfra {}

#[async_trait]
impl SubmissionFileStorage for SubmissionFileStorageInfra {
    async fn upload<B: bytes::Buf + Send>(
        &self,
        class_id: &ClassID,
        user_id: &UserID,
        buf: &mut B,
    ) -> StorageResult<String> {
        let file = SubmissionFileStorageFile::default();
        file.upload(class_id, user_id, buf).await
    }

    async fn create_submissions_zip(
        &self,
        class_id: &ClassID,
        submissions: &[SubmissionWithUserCode],
    ) -> StorageResult<String> {
        let file = SubmissionFileStorageFile::default();
        file.create_submissions_zip(class_id, submissions).await
    }
}
