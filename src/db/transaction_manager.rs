use sqlx::{Pool, Postgres, Transaction};
use std::sync::Arc;
use crate::error::db_error::DbError;

#[derive(Clone)]
pub struct TransactionManager {
    pool: Arc<Pool<Postgres>>,
}

impl TransactionManager {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    pub async fn begin_tx(&self) -> Result<Transaction<'static, Postgres>, DbError> {
        self.pool
            .begin()
            .await
            .map_err(|e| DbError::SomethingWentWrong(e.to_string()))
    }
} 