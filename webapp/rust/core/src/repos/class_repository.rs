use crate::db::{DBPool, TxConn};
use crate::models::class::{Class, ClassWithSubmitted, CreateClass};
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ClassRepository {
    async fn for_update_by_id_in_tx<'c>(&self, tx: &mut TxConn<'c>, id: &str) -> Result<bool>;
    async fn create_in_tx<'c>(&self, tx: &mut TxConn<'c>, class: &CreateClass) -> Result<()>;
    async fn update_submission_closed_by_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        id: &str,
    ) -> Result<()>;
    async fn find_submission_closed_by_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        id: &str,
    ) -> Result<Option<bool>>;
    async fn find_by_course_id_and_part(
        &self,
        pool: &DBPool,
        course_id: &str,
        part: &u8,
    ) -> Result<Class>;
    async fn find_all_by_course_id(&self, pool: &DBPool, course_id: &str) -> Result<Vec<Class>>;
    async fn find_all_with_submitteed_by_user_id_and_course_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
        course_id: &str,
    ) -> Result<Vec<ClassWithSubmitted>>;
}
