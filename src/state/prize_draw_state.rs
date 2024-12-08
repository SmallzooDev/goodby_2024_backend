use crate::config::database::Database;
use crate::repository::prize_draw_repository::PrizeDrawRepository;
use crate::repository::prize_repository::PrizeRepository;
use crate::repository::user_ticket_repository::UserTicketRepository;
use crate::service::prize_draw_service::PrizeDrawService;
use std::sync::Arc;

#[derive(Clone)]
pub struct PrizeDrawState {
    pub prize_draw_service: PrizeDrawService,
    #[allow(dead_code)]
    pub prize_draw_repo: PrizeDrawRepository,
    #[allow(dead_code)]
    pub prize_repo: PrizeRepository,
    #[allow(dead_code)]
    pub user_ticket_repo: UserTicketRepository,
}

impl PrizeDrawState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        let prize_draw_repo = PrizeDrawRepository::new(Arc::clone(db_conn));
        let prize_repo = PrizeRepository::new(Arc::clone(db_conn));
        let user_ticket_repo = UserTicketRepository::new(Arc::clone(db_conn));
        
        Self {
            prize_draw_service: PrizeDrawService::new(
                prize_draw_repo.clone(),
                prize_repo.clone(),
                user_ticket_repo.clone(),
            ),
            prize_draw_repo,
            prize_repo,
            user_ticket_repo,
        }
    }
} 