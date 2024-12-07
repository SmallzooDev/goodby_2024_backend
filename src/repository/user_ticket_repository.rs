use crate::config::database::{Database, DatabaseTrait};
use crate::dto::ticket_creation_result::TicketCreationResult;
use crate::dto::user_ticket_count::UserTicketCount;
use crate::dto::available_ticket::AvailableTicket;
use async_trait::async_trait;
use sqlx::{Error, PgConnection};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserTicketRepository {
    pub(crate) db_conn: Arc<Database>,
}

#[async_trait]
pub trait UserTicketRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    async fn get_ticket_ranking(&self) -> Result<Vec<UserTicketCount>, Error>;
    async fn create_ticket_in_tx(
        &self,
        tx: &mut PgConnection,
        user_id: i32,
    ) -> Result<TicketCreationResult, Error>;
    async fn get_available_tickets_for_draw(
        &self,
        tx: &mut PgConnection,
        count: i64,
    ) -> Result<Vec<AvailableTicket>, Error>;
    async fn mark_tickets_as_used(
        &self,
        tx: &mut PgConnection,
        ticket_numbers: &[String],
    ) -> Result<(), Error>;
}

#[async_trait]
impl UserTicketRepositoryTrait for UserTicketRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    async fn get_ticket_ranking(&self) -> Result<Vec<UserTicketCount>, Error> {
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
            .await?;

        Ok(result)
    }

    async fn create_ticket_in_tx(
        &self,
        tx: &mut PgConnection,
        user_id: i32,
    ) -> Result<TicketCreationResult, Error> {
        let ticket_number: i64 = sqlx::query_scalar!(
            r#"
            SELECT nextval('ticket_number_seq') as ticket_number
            "#
        )
            .fetch_one(&mut *tx)
            .await?
            .expect("Failed to generate ticket number");

        let ticket_number_str = ticket_number.to_string();

        sqlx::query!(
            r#"
            INSERT INTO user_tickets (user_id, ticket_number, available)
            VALUES ($1, $2, true)
            "#,
            user_id,
            ticket_number_str
        )
            .execute(&mut *tx)
            .await?;

        let result = TicketCreationResult {
            user_id,
            ticket_number: ticket_number_str,
            message: format!("Ticket created successfully for user {}", user_id),
            available: true,
        };

        Ok(result)
    }

    async fn get_available_tickets_for_draw(
        &self,
        tx: &mut PgConnection,
        count: i64,
    ) -> Result<Vec<AvailableTicket>, Error> {
        let available_tickets = sqlx::query_as!(
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
        .fetch_all(&mut *tx)
        .await?;

        Ok(available_tickets)
    }

    async fn mark_tickets_as_used(
        &self,
        tx: &mut PgConnection,
        ticket_numbers: &[String],
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE user_tickets
            SET available = false
            WHERE ticket_number = ANY($1)
            "#,
            ticket_numbers
        )
        .execute(&mut *tx)
        .await?;

        Ok(())
    }
}