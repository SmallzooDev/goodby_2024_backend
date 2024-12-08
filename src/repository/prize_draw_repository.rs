use crate::config::database::{Database, DatabaseTrait};
use crate::entity::prize_draw::PrizeDraw;
use async_trait::async_trait;
use sqlx::{Error, PgConnection};
use std::sync::Arc;

#[derive(Clone)]
pub struct PrizeDrawRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait PrizeDrawRepositoryTrait: Send + Sync {
    async fn begin_tx(&self) -> Result<sqlx::Transaction<'static, sqlx::Postgres>, Error>;
    async fn create_draw_in_tx(
        &self,
        tx: &mut PgConnection,
        prize_id: i32,
        prize_name: String,
        user_id: i32,
        user_name: String,
        department_name: String,
        ticket_number: String,
    ) -> Result<PrizeDraw, Error>;
    
    async fn find_all(&self) -> Result<Vec<PrizeDraw>, Error>;
    async fn find_by_prize_id(&self, prize_id: i32) -> Result<Vec<PrizeDraw>, Error>;
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
    async fn begin_tx(&self) -> Result<sqlx::Transaction<'static, sqlx::Postgres>, Error> {
        self.db_conn.get_pool().begin().await
    }

    async fn create_draw_in_tx(
        &self,
        tx: &mut PgConnection,
        prize_id: i32,
        prize_name: String,
        user_id: i32,
        user_name: String,
        department_name: String,
        ticket_number: String,
    ) -> Result<PrizeDraw, Error> {
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
        .fetch_one(&mut *tx)
        .await
    }

    async fn find_all(&self) -> Result<Vec<PrizeDraw>, Error> {
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
    }

    async fn find_by_prize_id(&self, prize_id: i32) -> Result<Vec<PrizeDraw>, Error> {
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
    }
} 