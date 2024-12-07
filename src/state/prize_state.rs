use crate::config::database::Database;
use crate::repository::prize_repository::{PrizeRepository, PrizeRepositoryTrait};
use crate::service::prize_service::PrizeService;
use std::sync::Arc;

#[derive(Clone)]
pub struct PrizeState {
    pub prize_service: PrizeService,
    pub prize_repo: PrizeRepository,
}

impl PrizeState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            prize_service: PrizeService::new(db_conn),
            prize_repo: PrizeRepository::new(db_conn),
        }
    }
} 