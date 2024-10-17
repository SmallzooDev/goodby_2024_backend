use crate::config::database::{Database};
use std::sync::Arc;
use crate::repository::user_ticket_repository::{UserTicketRepository, UserTicketRepositoryTrait};

#[derive(Clone)]
pub struct UserTicketService {
    user_ticket_repo: UserTicketRepository,
    db_conn: Arc<Database>,
}

impl UserTicketService {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_ticket_repo: UserTicketRepository::new(db_conn),
            db_conn: Arc::clone(db_conn),
        }
    }
}