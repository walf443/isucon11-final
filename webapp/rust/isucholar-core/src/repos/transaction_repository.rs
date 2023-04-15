use crate::db::{DBPool, TxConn};
use crate::repos::error::Result;
use async_trait::async_trait;

#[cfg_attr(any(test, feature = "test"), mockall::automock)]
#[async_trait]
pub trait TransactionRepository {
    async fn begin<'c>(&self, pool: &DBPool) -> Result<TxConn<'c>>;
}

pub trait HaveTransactionRepository {
    type Repo: TransactionRepository + Sync;
    fn transaction_repository(&self) -> &Self::Repo;
}

#[derive(Clone)]
pub struct TransactionRepositoryImpl {}

#[async_trait]
impl TransactionRepository for TransactionRepositoryImpl {
    async fn begin<'c>(&self, pool: &DBPool) -> Result<TxConn<'c>> {
        let txn = pool.begin().await?;
        Ok(txn)
    }
}
