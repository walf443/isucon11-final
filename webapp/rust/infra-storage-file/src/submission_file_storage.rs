use async_trait::async_trait;
use isucholar_core::models::class::ClassID;
use isucholar_core::models::submission::SubmissionWithUserCode;
use isucholar_core::models::user::UserID;
use isucholar_core::storages::submission_file_storage::SubmissionFileStorage;
use isucholar_core::storages::StorageResult;
use isucholar_core::ASSIGNMENTS_DIRECTORY;
use tokio::io::AsyncWriteExt;

#[derive(Default)]
pub struct SubmissionFileStorageFile {}

impl SubmissionFileStorageFile {
    pub fn get_filename(&self, class_id: &ClassID, user_id: &UserID) -> String {
        let dst = format!("{}{}-{}.pdf", ASSIGNMENTS_DIRECTORY, class_id, user_id,);
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

    async fn create_submissions_zip(
        &self,
        class_id: &ClassID,
        submissions: &[SubmissionWithUserCode],
    ) -> StorageResult<String> {
        let zip_file_path = format!("{}{}.zip", ASSIGNMENTS_DIRECTORY, class_id);

        let tmp_dir = format!("{}{}/", ASSIGNMENTS_DIRECTORY, class_id);
        tokio::process::Command::new("rm")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .arg("-rf")
            .arg(&tmp_dir)
            .status()
            .await?;
        tokio::process::Command::new("mkdir")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .arg(&tmp_dir)
            .status()
            .await?;

        // ファイル名を指定の形式に変更
        for submission in submissions {
            tokio::process::Command::new("cp")
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .arg(self.get_filename(class_id, &submission.user_id))
                .arg(&format!(
                    "{}{}-{}",
                    tmp_dir, submission.user_code, submission.file_name
                ))
                .status()
                .await?;
        }

        // -i 'tmp_dir/*': 空zipを許す
        tokio::process::Command::new("zip")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .arg("-j")
            .arg("-r")
            .arg(&zip_file_path)
            .arg(&tmp_dir)
            .arg("-i")
            .arg(&format!("{}*", tmp_dir))
            .status()
            .await?;

        Ok(zip_file_path.to_string())
    }
}
