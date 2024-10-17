use crate::config::database::Database;
use std::sync::Arc;
use crate::repository::user_ticket_repository::{UserTicketRepository, UserTicketRepositoryTrait};
use crate::service::user_ticket_service::UserTicketService;

#[derive(Clone)]
pub struct UserTicketState {
    pub user_ticket_service: UserTicketService,
    pub user_ticket_repo: UserTicketRepository,
}

impl UserTicketState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_ticket_service: UserTicketService::new(db_conn),
            user_ticket_repo: UserTicketRepository::new(db_conn),
        }
    }
}
