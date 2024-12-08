use crate::config::database::Database;
use crate::repository::user_ticket_repository::UserTicketRepository;
use crate::service::user_ticket_service::UserTicketService;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserTicketState {
    pub user_ticket_service: UserTicketService,
    #[allow(dead_code)]
    pub user_ticket_repo: UserTicketRepository,
}

impl UserTicketState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        let user_ticket_repo = UserTicketRepository::new(Arc::clone(db_conn));
        Self {
            user_ticket_service: UserTicketService::new(user_ticket_repo.clone()),
            user_ticket_repo,
        }
    }
}
