use crate::config::database::{Database, DatabaseTrait};
use crate::entity::prize_draw::PrizeDraw;
use crate::error::db_error::DbError;
use async_trait::async_trait;
use sqlx::{PgConnection, Postgres, Transaction};
use std::sync::Arc;

#[derive(Clone)]
pub struct PrizeDrawRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait PrizeDrawRepositoryTrait: Send + Sync {
    async fn create_draw_in_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        prize_id: i32,
        prize_name: String,
        user_id: i32,
        user_name: String,
        department_name: String,
        ticket_number: String,
    ) -> Result<PrizeDraw, DbError>;
    
    async fn find_all(&self) -> Result<Vec<PrizeDraw>, DbError>;
    async fn find_by_prize_id(&self, prize_id: i32) -> Result<Vec<PrizeDraw>, DbError>;
}

impl PrizeDrawRepository {
    pub fn new(db_conn: Arc<Database>) -> Self {
        Self {
            db_conn
        }
    }
}

#[async_trait]
impl PrizeDrawRepositoryTrait for PrizeDrawRepository {
    async fn create_draw_in_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        prize_id: i32,
        prize_name: String,
        user_id: i32,
        user_name: String,
        department_name: String,
        ticket_number: String,
    ) -> Result<PrizeDraw, DbError> {
        sqlx::query_as!(
            PrizeDraw,
            r#"
            INSERT INTO prize_draws (prize_id, prize_name, user_id, user_name, department_name, ticket_number)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, prize_id, prize_name, user_id, user_name, department_name, ticket_number, created_at
            "#,
            prize_id,
            prize_name,
            user_id,
            user_name,
            department_name,
            ticket_number
        )
        .fetch_one(tx as &mut PgConnection)
        .await
        .map_err(|e| DbError::SomethingWentWrong(e.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<PrizeDraw>, DbError> {
        sqlx::query_as!(
            PrizeDraw,
            r#"
            SELECT id, prize_id, prize_name, user_id, user_name, department_name, ticket_number, created_at
            FROM prize_draws
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(self.db_conn.get_pool())
        .await
        .map_err(|e| DbError::SomethingWentWrong(e.to_string()))
    }

    async fn find_by_prize_id(&self, prize_id: i32) -> Result<Vec<PrizeDraw>, DbError> {
        sqlx::query_as!(
            PrizeDraw,
            r#"
            SELECT id, prize_id, prize_name, user_id, user_name, department_name, ticket_number, created_at
            FROM prize_draws
            WHERE prize_id = $1
            ORDER BY created_at DESC
            "#,
            prize_id
        )
        .fetch_all(self.db_conn.get_pool())
        .await
        .map_err(|e| DbError::SomethingWentWrong(e.to_string()))
    }
} 