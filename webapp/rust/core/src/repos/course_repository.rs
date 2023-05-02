use crate::db::{DBConn, DBPool};
use crate::models::course::{Course, CourseCode, CourseID, CourseWithTeacher, CreateCourse};
use crate::models::course_status::CourseStatus;
use crate::models::day_of_week::DayOfWeek;
use crate::repos::error::Result;
use async_trait::async_trait;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SearchCoursesQuery {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub credit: Option<i64>,
    pub teacher: Option<String>,
    pub period: Option<i64>,
    pub day_of_week: Option<DayOfWeek>,
    pub keywords: Option<String>,
    pub status: Option<String>,
    pub page: Option<String>,
}

#[async_trait]
pub trait CourseRepository {
    async fn create(&self, pool: &DBPool, course: &CreateCourse) -> Result<CourseID>;
    async fn find_all_with_teacher(
        &self,
        pool: &DBPool,
        limit: i64,
        offset: i64,
        query: &SearchCoursesQuery,
    ) -> Result<Vec<CourseWithTeacher>>;
    async fn find_status_for_share_lock_by_id(
        &self,
        conn: &mut DBConn,
        id: &CourseID,
    ) -> Result<Option<CourseStatus>>;
    async fn find_for_share_lock_by_id(
        &self,
        conn: &mut DBConn,
        id: &CourseID,
    ) -> Result<Option<Course>>;
    async fn exist_by_id(&self, conn: &mut DBConn, id: &CourseID) -> Result<bool>;
    async fn for_update_by_id(&self, conn: &mut DBConn, id: &CourseID) -> Result<bool>;
    async fn update_status_by_id(
        &self,
        conn: &mut DBConn,
        id: &CourseID,
        status: &CourseStatus,
    ) -> Result<()>;
    async fn find_by_code(&self, conn: &mut DBConn, code: &CourseCode) -> Result<Course>;
    async fn find_with_teacher_by_id(
        &self,
        conn: &mut DBConn,
        id: &CourseID,
    ) -> Result<Option<CourseWithTeacher>>;
}

pub trait HaveCourseRepository {
    type Repo: CourseRepository + Sync;

    fn course_repo(&self) -> &Self::Repo;
}
