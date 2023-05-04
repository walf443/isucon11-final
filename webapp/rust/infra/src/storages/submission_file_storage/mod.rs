use async_trait::async_trait;
use isucholar_core::models::class::ClassID;
use isucholar_core::models::user::UserID;
use isucholar_core::storages::submission_file_storage::SubmissionFileStorage;
use isucholar_core::storages::StorageResult;
use isucholar_infra_storage_file::submission_file_storage::SubmissionFileStorageFile;

pub struct SubmissionFileStorageInfra {}

impl SubmissionFileStorageInfra {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SubmissionFileStorage for SubmissionFileStorageInfra {
    async fn upload<B: bytes::Buf + Send>(
        &self,
        class_id: &ClassID,
        user_id: &UserID,
        buf: &mut B,
    ) -> StorageResult<String> {
        let file = SubmissionFileStorageFile::new();
        file.upload(class_id, user_id, buf).await
    }
}
