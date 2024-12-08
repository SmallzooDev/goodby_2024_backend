use crate::config::database::{Database, DatabaseTrait};
use crate::dto::ticket_creation_result::TicketCreationResult;
use crate::dto::user_ticket_count::UserTicketCount;
use crate::dto::available_ticket::AvailableTicket;
use crate::error::db_error::DbError;
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use std::sync::Arc;
use sqlx::postgres::PgConnection;

#[derive(Clone)]
pub struct UserTicketRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait UserTicketRepositoryTrait: Send + Sync {
    async fn get_ticket_ranking(&self) -> Result<Vec<UserTicketCount>, DbError>;
    async fn create_ticket_in_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: i32,
    ) -> Result<TicketCreationResult, DbError>;
    async fn get_available_tickets_for_draw(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        count: i64,
    ) -> Result<Vec<AvailableTicket>, DbError>;
    async fn mark_tickets_as_used(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        ticket_numbers: &[String],
    ) -> Result<(), DbError>;
}

impl UserTicketRepository {
    pub fn new(db_conn: Arc<Database>) -> Self {
        Self {
            db_conn
        }
    }
}

#[async_trait]
impl UserTicketRepositoryTrait for UserTicketRepository {
    async fn get_ticket_ranking(&self) -> Result<Vec<UserTicketCount>, DbError> {
        let result = sqlx::query_as!(
            UserTicketCount,
            r#"
            SELECT u.id AS user_id, u.name, COUNT(*) AS ticket_count
            FROM user_tickets ut
            JOIN users u ON ut.user_id = u.id
            GROUP BY u.id, u.name
            ORDER BY ticket_count DESC
        "#
        )
            .fetch_all(self.db_conn.get_pool())
            .await
            .map_err(|e| DbError::SomethingWentWrong(e.to_string()))?;

        Ok(result)
    }

    async fn create_ticket_in_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: i32,
    ) -> Result<TicketCreationResult, DbError> {
        let ticket_number: i64 = sqlx::query_scalar!(
            r#"
            SELECT nextval('ticket_number_seq') as ticket_number
            "#
        )
            .fetch_one(tx as &mut PgConnection)
            .await
            .map_err(|e| DbError::SomethingWentWrong(e.to_string()))?
            .expect("Failed to generate ticket number");

        sqlx::query!(
            r#"
            INSERT INTO user_tickets (user_id, ticket_number, available)
            VALUES ($1, $2, true)
            "#,
            user_id,
            ticket_number.to_string(),
        )
            .execute(tx as &mut PgConnection)
            .await
            .map_err(|e| DbError::SomethingWentWrong(e.to_string()))?;

        Ok(TicketCreationResult {
            user_id,
            ticket_number: ticket_number.to_string(),
            message: format!("Ticket created successfully for user {}", user_id),
            available: true,
        })
    }

    async fn get_available_tickets_for_draw(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        count: i64,
    ) -> Result<Vec<AvailableTicket>, DbError> {
        sqlx::query_as!(
            AvailableTicket,
            r#"
            SELECT 
                u.id as "user_id",
                u.name as "user_name", 
                t.team_name as "department_name",
                ut.ticket_number as "ticket_number"
            FROM user_tickets ut
            JOIN users u ON ut.user_id = u.id
            JOIN team t ON u.team_id = t.id
            WHERE ut.available = true
            ORDER BY RANDOM()
            LIMIT $1
            FOR UPDATE
            "#,
            count
        )
        .fetch_all(tx as &mut PgConnection)
        .await
        .map_err(|e| DbError::SomethingWentWrong(e.to_string()))
    }

    async fn mark_tickets_as_used(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        ticket_numbers: &[String],
    ) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            UPDATE user_tickets
            SET available = false
            WHERE ticket_number = ANY($1)
            "#,
            ticket_numbers
        )
        .execute(tx as &mut PgConnection)
        .await
        .map(|_| ())
        .map_err(|e| DbError::SomethingWentWrong(e.to_string()))
    }
}