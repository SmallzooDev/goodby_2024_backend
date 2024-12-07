use crate::config::database::Database;
use crate::repository::prize_repository::{PrizeRepository, PrizeRepositoryTrait};
use crate::service::prize_service::PrizeService;
use crate::service::prize_draw_service::PrizeDrawService;
use std::sync::Arc;

#[derive(Clone)]
pub struct PrizeState {
    pub prize_service: PrizeService,
    pub prize_repo: PrizeRepository,
    pub prize_draw_service: PrizeDrawService,
}

impl PrizeState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            prize_service: PrizeService::new(db_conn),
            prize_repo: PrizeRepository::new(db_conn),
            prize_draw_service: PrizeDrawService::new(db_conn),
        }
    }
} 