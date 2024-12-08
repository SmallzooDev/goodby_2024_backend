use crate::config::database::{Database, DatabaseTrait};
use crate::db::transaction_manager::TransactionManager;
use crate::repository::user_ticket_repository::UserTicketRepository;
use crate::service::user_ticket_service::UserTicketService;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserTicketState {
    pub service: Arc<UserTicketService>,
}

impl UserTicketState {
    pub fn new(db: Arc<Database>) -> Self {
        let tx_manager = TransactionManager::new(db.get_pool().clone());
        let user_ticket_repo = UserTicketRepository::new(db);
        
        let service = UserTicketService::new(
            tx_manager,
            user_ticket_repo,
        );

        Self {
            service: Arc::new(service),
        }
    }
}
