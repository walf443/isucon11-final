use crate::db::{DBPool, TxConn};
use crate::repos::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionRepository {
    async fn begin(&self, pool: &DBPool) -> Result<TxConn>;
}

pub trait HaveTransactionRepository {
    type Repo: TransactionRepository + Sync;
    fn transaction_repository(&self) -> &Self::Repo;
}

pub struct TransactionRepositoryImpl {}

#[async_trait]
impl TransactionRepository for TransactionRepositoryImpl {
    async fn begin(&self, pool: &DBPool) -> Result<TxConn> {
        let txn = pool.begin().await?;
        Ok(txn)
    }
}
