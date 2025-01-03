use crate::dto::ticket_creation_result::TicketCreationResult;
use crate::dto::user_ticket_count::UserTicketCount;
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::repository::user_ticket_repository::UserTicketRepositoryTrait;
use crate::db::transaction_manager::TransactionManager;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserTicketService {
    tx_manager: Arc<TransactionManager>,
    user_ticket_repo: Arc<dyn UserTicketRepositoryTrait>,
}

impl UserTicketService {
    pub fn new(
        tx_manager: TransactionManager,
        user_ticket_repo: impl UserTicketRepositoryTrait + 'static,
    ) -> Self {
        Self {
            tx_manager: Arc::new(tx_manager),
            user_ticket_repo: Arc::new(user_ticket_repo),
        }
    }

    pub async fn create_tickets_for_users(
        &self,
        user_ids: Vec<i32>,
    ) -> Result<Vec<TicketCreationResult>, ApiError> {
        let mut tx = self
            .tx_manager
            .begin_tx()
            .await
            .map_err(ApiError::Db)?;

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

