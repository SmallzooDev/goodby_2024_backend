use crate::config::database::Database;
use crate::repository::prize_repository::PrizeRepository;
use crate::repository::user_ticket_repository::UserTicketRepository;
use crate::repository::prize_draw_repository::PrizeDrawRepository;
use crate::service::prize_service::PrizeService;
use crate::service::prize_draw_service::PrizeDrawService;
use std::sync::Arc;

#[derive(Clone)]
pub struct PrizeState {
    pub prize_service: PrizeService,
    #[allow(dead_code)]
    pub prize_repo: PrizeRepository,
    pub prize_draw_service: PrizeDrawService,
    #[allow(dead_code)]
    pub prize_draw_repo: PrizeDrawRepository,
    #[allow(dead_code)]
    pub user_ticket_repo: UserTicketRepository,
}

impl PrizeState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        let prize_repo = PrizeRepository::new(Arc::clone(db_conn));
        let prize_draw_repo = PrizeDrawRepository::new(Arc::clone(db_conn));
        let user_ticket_repo = UserTicketRepository::new(Arc::clone(db_conn));

        Self {
            prize_service: PrizeService::new(prize_repo.clone()),
            prize_draw_service: PrizeDrawService::new(
                prize_draw_repo.clone(),
                prize_repo.clone(),
                user_ticket_repo.clone(),
            ),
            prize_repo,
            prize_draw_repo,
            user_ticket_repo,
        }
    }
} 