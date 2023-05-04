use async_trait::async_trait;
use isucholar_core::models::class::ClassID;
use isucholar_core::models::user::UserID;
use isucholar_core::storages::submission_file_storage::SubmissionFileStorage;
use isucholar_core::storages::StorageResult;
use isucholar_core::ASSIGNMENTS_DIRECTORY;
use tokio::io::AsyncWriteExt;

pub struct SubmissionFileStorageFile {}

impl SubmissionFileStorageFile {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_filename(&self, class_id: &ClassID, user_id: &UserID) -> String {
        let dst = format!(
            "{}{}-{}.pdf",
            ASSIGNMENTS_DIRECTORY,
            class_id.to_string(),
            user_id.to_string(),
        );
        dst
    }
}

#[async_trait]
impl SubmissionFileStorage for SubmissionFileStorageFile {
    async fn upload<B: bytes::Buf + Send>(
        &self,
        class_id: &ClassID,
        user_id: &UserID,
        buf: &mut B,
    ) -> StorageResult<String> {
        let dst = self.get_filename(class_id, user_id);
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o666)
            .open(&dst)
            .await?;
        file.write_all_buf(buf).await?;

        Ok(dst)
    }
}
