use crate::config::database::DatabaseTrait;
use crate::dto::ticket_creation_result::TicketCreationResult;
use crate::dto::user_ticket_count::UserTicketCount;
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::repository::user_ticket_repository::{UserTicketRepository, UserTicketRepositoryTrait};

#[derive(Clone)]
pub struct UserTicketService {
    user_ticket_repo: UserTicketRepository,
}

impl UserTicketService {
    pub fn new(user_ticket_repo: UserTicketRepository) -> Self {
        Self {
            user_ticket_repo
        }
    }

    pub async fn create_tickets_for_users(
        &self,
        user_ids: Vec<i32>,
    ) -> Result<Vec<TicketCreationResult>, ApiError> {
        let mut tx = self
            .user_ticket_repo
            .db_conn
            .get_pool()
            .begin()
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        let mut results = Vec::new();

        for user_id in user_ids {
            let ticket_result = self
                .user_ticket_repo
                .create_ticket_in_tx(&mut tx, user_id)
                .await
                .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

            results.push(ticket_result);
        }

        tx.commit()
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        Ok(results)
    }

    pub async fn get_ticket_ranking(&self) -> Result<Vec<UserTicketCount>, ApiError> {
        let rankings = self
            .user_ticket_repo
            .get_ticket_ranking()
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;
        Ok(rankings)
    }
}

