#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait SubmissionService {
    async fn create_or_update<B: bytes::Buf + Send + 'static>(
        &self,
        user_id: &UserID,
        course_id: &CourseID,
        class_id: &ClassID,
        file_name: &str,
        data: &mut B,
    ) -> Result<()>;

    async fn download_submissions_zip(&self, class_id: &ClassID) -> Result<String>;

    async fn update_user_scores_by_class_id(
        &self,
        class_id: &ClassID,
        scores: &Vec<Score>,
    ) -> Result<()>;
}

use crate::models::class::ClassID;
use crate::models::course::CourseID;
use crate::models::course_status::CourseStatus;
use crate::models::score::Score;
use crate::models::submission::CreateSubmission;
use crate::models::user::{UserCode, UserID};
use crate::repos::class_repository::{ClassRepository, HaveClassRepository};
use crate::repos::course_repository::{CourseRepository, HaveCourseRepository};
use crate::repos::registration_repository::{HaveRegistrationRepository, RegistrationRepository};
use crate::repos::submission_repository::{HaveSubmissionRepository, SubmissionRepository};
use crate::services::error::Error::{
    ClassIsNotSubmissionClosed, ClassNotFound, CourseIsNotInProgress, CourseNotFound,
    RegistrationAlready, SubmissionClosed,
};
use crate::services::error::Result;
use crate::services::HaveDBPool;
use crate::storages::submission_file_storage::{HaveSubmissionFileStorage, SubmissionFileStorage};
use async_trait::async_trait;
use bytes::Buf;

pub trait HaveSubmissionService {
    type Service: SubmissionService;
    fn submission_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait SubmissionServiceImpl:
    Sync
    + HaveDBPool
    + HaveClassRepository
    + HaveSubmissionRepository
    + HaveCourseRepository
    + HaveRegistrationRepository
    + HaveSubmissionFileStorage
{
    async fn create_or_update<B: bytes::Buf + Send>(
        &self,
        user_id: &UserID,
        course_id: &CourseID,
        class_id: &ClassID,
        file_name: &str,
        data: &mut B,
    ) -> Result<()> {
        let pool = self.get_db_pool();
        let mut tx = pool.begin().await?;
        let course_repo = self.course_repo();
        let status = course_repo
            .find_status_for_share_lock_by_id(&mut tx, &course_id)
            .await?;
        if let Some(status) = status {
            if status != CourseStatus::InProgress {
                return Err(CourseIsNotInProgress);
            }
        } else {
            return Err(CourseNotFound);
        }

        let registration_repo = self.registration_repo();

        let is_registered = registration_repo
            .exist_by_user_id_and_course_id(&mut tx, &user_id, &course_id)
            .await?;
        if is_registered {
            return Err(RegistrationAlready);
        }

        let class_repo = self.class_repo();
        let submission_closed = class_repo
            .find_submission_closed_by_id_with_shared_lock(&mut tx, &class_id)
            .await?;

        if let Some(submission_closed) = submission_closed {
            if submission_closed {
                return Err(SubmissionClosed);
            }
        } else {
            return Err(ClassNotFound);
        }

        let submission_repo = self.submission_repo();
        submission_repo
            .create_or_update(
                &mut tx,
                &CreateSubmission {
                    file_name: file_name.to_string(),
                    user_id: user_id.clone(),
                    class_id: class_id.clone(),
                },
            )
            .await?;

        let submission_file_storage = self.submission_file_storage();
        submission_file_storage
            .upload(&class_id, &user_id, data)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn download_submissions_zip(&self, class_id: &ClassID) -> Result<String> {
        let pool = self.get_db_pool();

        let mut tx = pool.begin().await?;
        let class_repo = self.class_repo();
        let is_exist = class_repo.for_update_by_id(&mut tx, &class_id).await?;

        if !is_exist {
            return Err(ClassNotFound);
        }
        let submission_repo = self.submission_repo();
        let submissions = submission_repo
            .find_all_with_user_code_by_class_id(&mut tx, &class_id)
            .await?;

        let submission_file_storage = self.submission_file_storage();
        let zip_file_path = submission_file_storage
            .create_submissions_zip(&class_id, &submissions)
            .await?;

        class_repo
            .update_submission_closed_by_id(&mut tx, &class_id)
            .await?;

        tx.commit().await?;

        Ok(zip_file_path)
    }

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
    async fn create_or_update<B: Buf + Send + 'static>(
        &self,
        user_id: &UserID,
        course_id: &CourseID,
        class_id: &ClassID,
        file_name: &str,
        data: &mut B,
    ) -> Result<()> {
        SubmissionServiceImpl::create_or_update(self, user_id, course_id, class_id, file_name, data)
            .await
    }

    async fn download_submissions_zip(&self, class_id: &ClassID) -> Result<String> {
        SubmissionServiceImpl::download_submissions_zip(self, class_id).await
    }

    async fn update_user_scores_by_class_id(
        &self,
        class_id: &ClassID,
        scores: &Vec<Score>,
    ) -> Result<()> {
        SubmissionServiceImpl::update_user_scores_by_class_id(self, class_id, scores).await
    }
}
