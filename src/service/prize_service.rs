use crate::config::database::Database;
use crate::dto::prize_dto::{CreatePrizeDto, PrizeDto};
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::repository::prize_repository::{PrizeRepository, PrizeRepositoryTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct PrizeService {
    prize_repo: PrizeRepository,
}

impl PrizeService {
    pub fn new(prize_repo: PrizeRepository) -> Self {
        Self {
            prize_repo
        }
    }

    pub async fn create_prize(&self, payload: CreatePrizeDto) -> Result<PrizeDto, ApiError> {
        let prize = self
            .prize_repo
            .create(payload.name, payload.stock)
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        Ok(PrizeDto {
            id: prize.id,
            name: prize.name,
            stock: prize.stock,
        })
    }

    pub async fn get_all_prizes(&self) -> Result<Vec<PrizeDto>, ApiError> {
        let prizes = self
            .prize_repo
            .find_all()
            .await
            .map_err(|e| ApiError::Db(DbError::SomethingWentWrong(e.to_string())))?;

        Ok(prizes
            .into_iter()
            .map(|p| PrizeDto {
                id: p.id,
                name: p.name,
                stock: p.stock,
            })
            .collect())
    }
} 