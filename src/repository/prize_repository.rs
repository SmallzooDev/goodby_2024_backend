use crate::config::database::{Database, DatabaseTrait};
use crate::entity::prize::Prize;
use async_trait::async_trait;
use sqlx::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct PrizeRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait PrizeRepositoryTrait: Send + Sync {
    async fn create(&self, name: String, stock: i32) -> Result<Prize, Error>;
    async fn find_all(&self) -> Result<Vec<Prize>, Error>;
    async fn find_by_id_in_tx(&self, tx: &mut sqlx::PgConnection, id: i32) -> Result<Prize, Error>;
}

impl PrizeRepository {
    pub fn new(db_conn: Arc<Database>) -> Self {
        Self {
            db_conn
        }
    }
}

#[async_trait]
impl PrizeRepositoryTrait for PrizeRepository {
    async fn create(&self, name: String, stock: i32) -> Result<Prize, Error> {
        sqlx::query_as!(
            Prize,
            r#"
            INSERT INTO prizes (name, stock)
            VALUES ($1, $2)
            RETURNING id, name, stock
            "#,
            name,
            stock
        )
        .fetch_one(self.db_conn.get_pool())
        .await
    }

    async fn find_all(&self) -> Result<Vec<Prize>, Error> {
        sqlx::query_as!(
            Prize,
            "SELECT id, name, stock FROM prizes ORDER BY id"
        )
        .fetch_all(self.db_conn.get_pool())
        .await
    }

    async fn find_by_id_in_tx(&self, tx: &mut sqlx::PgConnection, id: i32) -> Result<Prize, Error> {
        sqlx::query_as!(
            Prize,
            "SELECT id, name, stock FROM prizes WHERE id = $1",
            id
        )
        .fetch_one(&mut *tx)
        .await
    }
} 