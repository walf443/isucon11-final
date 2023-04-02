use crate::db;
use crate::db::TxConn;
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UnreadAnnouncemnetRepository {
    async fn count_unread_by_user_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
    ) -> Result<i64>;
}

pub struct UnreadAnnouncemntRepositoryImpl {}

#[async_trait]
impl UnreadAnnouncemnetRepository for UnreadAnnouncemntRepositoryImpl {
    async fn count_unread_by_user_id_in_tx<'c>(
        &self,
        tx: &mut TxConn<'c>,
        user_id: &str,
    ) -> Result<i64> {
        let unread_count: i64 = db::fetch_one_scalar(
            sqlx::query_scalar(
                "SELECT COUNT(*) FROM `unread_announcements` WHERE `user_id` = ? AND NOT `is_deleted`",
            )
                .bind(user_id),
            tx,
        )
            .await?;

        Ok(unread_count)
    }
}
