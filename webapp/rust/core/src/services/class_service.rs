use crate::models::class::{ClassID, ClassWithSubmitted, CreateClass};
use crate::models::class_score::ClassScore;
use crate::models::course::{Course, CourseID};
use crate::models::course_result::CourseResult;
use crate::models::course_status::CourseStatus;
use crate::models::user::UserID;
use crate::repos::class_repository::{ClassRepository, HaveClassRepository};
use crate::repos::course_repository::{CourseRepository, HaveCourseRepository};
use crate::repos::error::ReposError;
use crate::repos::registration_course_repository::{
    HaveRegistrationCourseRepository, RegistrationCourseRepository,
};
use crate::repos::submission_repository::{HaveSubmissionRepository, SubmissionRepository};
use crate::services::error::Error::{CourseConflict, CourseIsNotInProgress, CourseNotFound};
use crate::services::error::Result;
use crate::services::HaveDBPool;
use crate::util;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait ClassService {
    async fn create(&self, form: &CreateClass) -> Result<ClassID>;

    async fn get_user_scores_by_course_id(
        &self,
        user_id: &UserID,
        course_id: &CourseID,
    ) -> Result<Vec<ClassScore>>;

    async fn get_user_course_result_by_course(
        &self,
        user_id: &UserID,
        course: &Course,
    ) -> Result<CourseResult>;
    async fn get_user_courses_result_by_courses(
        &self,
        user_id: &UserID,
        courses: &Vec<Course>,
    ) -> Result<(Vec<CourseResult>, f64, i64)>;
    async fn find_all_with_submitted_by_user_id_and_course_id<'c>(
        &self,
        user_id: &UserID,
        course_id: &CourseID,
    ) -> Result<Vec<ClassWithSubmitted>>;
}

pub trait HaveClassService {
    type Service: ClassService;

    fn class_service(&self) -> &Self::Service;
}

#[async_trait]
pub trait ClassServiceImpl:
    Sync
    + HaveDBPool
    + HaveClassRepository
    + HaveSubmissionRepository
    + HaveRegistrationCourseRepository
    + HaveCourseRepository
{
    async fn create(&self, form: &CreateClass) -> Result<ClassID> {
        let class_id = ClassID::new(util::new_ulid().await);

        let pool = self.get_db_pool();
        let mut tx = pool.begin().await?;

        let course = self
            .course_repo()
            .find_for_share_lock_by_id(&mut tx, &form.course_id)
            .await?;
        if course.is_none() {
            return Err(CourseNotFound);
        }
        let course = course.unwrap();
        if course.status != CourseStatus::InProgress {
            return Err(CourseIsNotInProgress);
        }

        let class_repo = self.class_repo();
        let result = class_repo.create(&mut tx, &class_id, form).await;
        match result {
            Ok(_) => {
                tx.commit().await?;
            }
            Err(e) => {
                let _ = tx.rollback().await;
                match e {
                    ReposError::CourseDuplicate => {
                        let mut conn = pool.acquire().await?;
                        let class = class_repo
                            .find_by_course_id_and_part(&mut conn, &form.course_id, &form.part)
                            .await?;
                        if form.title != class.title || form.description != class.description {
                            return Err(CourseConflict);
                        } else {
                            return Ok(class_id);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(class_id)
    }

    async fn get_user_scores_by_course_id(
        &self,
        user_id: &UserID,
        course_id: &CourseID,
    ) -> Result<Vec<ClassScore>> {
        let pool = self.get_db_pool();
        let mut conn = pool.acquire().await?;

        let classes = self
            .class_repo()
            .find_all_by_course_id(&mut conn, course_id)
            .await?;
        let mut class_scores = Vec::with_capacity(classes.len());

        let submission_repo = self.submission_repo();
        for class in classes {
            let submissions_count = submission_repo
                .count_by_class_id(&mut conn, &class.id)
                .await?;
            let my_score = submission_repo
                .find_score_by_class_id_and_user_id(&mut conn, &class.id, user_id)
                .await?;

            if let Some(my_score) = my_score {
                let my_score = my_score as i64;
                class_scores.push(ClassScore {
                    class_id: class.id,
                    part: class.part,
                    title: class.title,
                    score: Some(my_score),
                    submitters: submissions_count,
                });
            } else {
                class_scores.push(ClassScore {
                    class_id: class.id,
                    part: class.part,
                    title: class.title,
                    score: None,
                    submitters: submissions_count,
                });
            }
        }

        Ok(class_scores)
    }

    async fn get_user_course_result_by_course(
        &self,
        user_id: &UserID,
        course: &Course,
    ) -> Result<CourseResult> {
        let pool = self.get_db_pool();
        let mut conn = pool.acquire().await?;

        let class_scores = self
            .get_user_scores_by_course_id(user_id, &course.id)
            .await?;

        let mut my_total_score: i64 = 0;
        for score in &class_scores {
            if let Some(my_score) = score.score {
                my_total_score += my_score;
            }
        }

        let course_id = CourseID::new(course.id.to_string());
        let totals = self
            .registration_course_repo()
            .find_total_scores_by_course_id_group_by_user_id(&mut conn, &course_id)
            .await?;

        Ok(CourseResult {
            name: course.name.clone(),
            code: course.code.clone(),
            total_score: my_total_score,
            total_score_t_score: util::t_score_int(my_total_score, &totals),
            total_score_avg: util::average_int(&totals, 0.0),
            total_score_max: util::max_int(&totals, 0),
            total_score_min: util::min_int(&totals, 0),
            class_scores,
        })
    }

    async fn get_user_courses_result_by_courses(
        &self,
        user_id: &UserID,
        courses: &Vec<Course>,
    ) -> Result<(Vec<CourseResult>, f64, i64)> {
        // 科目毎の成績計算処理
        let mut course_results = Vec::with_capacity(courses.len());
        let mut my_gpa = 0f64;
        let mut my_credits = 0;

        for course in courses {
            let course_result = self
                .get_user_course_result_by_course(user_id, course)
                .await?;
            let my_total_score = course_result.total_score;
            course_results.push(course_result);

            // 自分のGPA計算
            if course.status == CourseStatus::Closed {
                my_gpa += (my_total_score * course.credit as i64) as f64;
                my_credits += course.credit as i64;
            }
        }
        if my_credits > 0 {
            my_gpa = my_gpa / 100.0 / my_credits as f64;
        }

        Ok((course_results, my_gpa, my_credits))
    }

    async fn find_all_with_submitted_by_user_id_and_course_id<'c>(
        &self,
        user_id: &UserID,
        course_id: &CourseID,
    ) -> Result<Vec<ClassWithSubmitted>> {
        let pool = self.get_db_pool();

        let mut tx = pool.begin().await?;
        let is_exist = self.course_repo().exist_by_id(&mut tx, course_id).await?;

        if !is_exist {
            return Err(CourseNotFound);
        }

        let classes = self
            .class_repo()
            .find_all_with_submitted_by_user_id_and_course_id(&mut tx, user_id, course_id)
            .await?;

        tx.commit().await?;

        Ok(classes)
    }
}

#[async_trait]
impl<S: ClassServiceImpl> ClassService for S {
    async fn create(&self, form: &CreateClass) -> Result<ClassID> {
        ClassServiceImpl::create(self, form).await
    }

    async fn get_user_scores_by_course_id(
        &self,
        user_id: &UserID,
        course_id: &CourseID,
    ) -> Result<Vec<ClassScore>> {
        ClassServiceImpl::get_user_scores_by_course_id(self, user_id, course_id).await
    }

    async fn get_user_course_result_by_course(
        &self,
        user_id: &UserID,
        course: &Course,
    ) -> Result<CourseResult> {
        ClassServiceImpl::get_user_course_result_by_course(self, user_id, course).await
    }

    async fn get_user_courses_result_by_courses(
        &self,
        user_id: &UserID,
        courses: &Vec<Course>,
    ) -> Result<(Vec<CourseResult>, f64, i64)> {
        ClassServiceImpl::get_user_courses_result_by_courses(self, user_id, courses).await
    }

    async fn find_all_with_submitted_by_user_id_and_course_id<'c>(
        &self,
        user_id: &UserID,
        course_id: &CourseID,
    ) -> Result<Vec<ClassWithSubmitted>> {
        ClassServiceImpl::find_all_with_submitted_by_user_id_and_course_id(self, user_id, course_id)
            .await
    }
}
