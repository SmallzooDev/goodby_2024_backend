use crate::dto::prize_dto::{CreatePrizeDto, PrizeDto};
use crate::error::api_error::ApiError;
use crate::state::prize_state::PrizeState;
use axum::{extract::State, Json};
use std::sync::Arc;

pub async fn create_prize_handler(
    State(state): State<Arc<PrizeState>>,
    Json(payload): Json<CreatePrizeDto>,
) -> Result<Json<PrizeDto>, ApiError> {
    let prize = state
        .prize_service
        .create_prize(payload)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(prize))
}

pub async fn get_prizes_handler(
    State(state): State<Arc<PrizeState>>,
) -> Result<Json<Vec<PrizeDto>>, ApiError> {
    let prizes = state
        .prize_service
        .get_all_prizes()
        .await
        .map_err(ApiError::from)?;
    Ok(Json(prizes))
} 