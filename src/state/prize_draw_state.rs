use crate::config::database::{Database, DatabaseTrait};
use crate::db::transaction_manager::TransactionManager;
use crate::repository::prize_draw_repository::PrizeDrawRepository;
use crate::repository::prize_repository::PrizeRepository;
use crate::repository::user_ticket_repository::UserTicketRepository;
use crate::service::prize_draw_service::PrizeDrawService;
use std::sync::Arc;

#[derive(Clone)]
pub struct PrizeDrawState {
    pub service: Arc<PrizeDrawService>,
}

impl PrizeDrawState {
    pub fn new(db: Arc<Database>) -> Self {
        let tx_manager = TransactionManager::new(db.get_pool().clone());
        
        let prize_draw_repo = PrizeDrawRepository::new(db.clone());
        let prize_repo = PrizeRepository::new(db.clone());
        let user_ticket_repo = UserTicketRepository::new(db);

        let service = PrizeDrawService::new(
            tx_manager,
            prize_draw_repo,
            prize_repo,
            user_ticket_repo,
        );

        Self {
            service: Arc::new(service),
        }
    }
} 