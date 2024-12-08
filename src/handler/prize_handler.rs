use crate::dto::prize_dto::{CreatePrizeDto, PrizeDto};
use crate::error::api_error::ApiError;
use crate::state::prize_state::PrizeState;
use axum::{extract::State, Json};
use std::sync::Arc;
use crate::dto::prize_draw_dto::{DrawPrizeRequestDto, PrizeDrawDto};

pub async fn create_prize_handler(
    State(state): State<Arc<PrizeState>>,
    Json(payload): Json<CreatePrizeDto>,
) -> Result<Json<PrizeDto>, ApiError> {
    let prize = state
        .service
        .create_prize(payload)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(prize))
}

pub async fn get_prizes_handler(
    State(state): State<Arc<PrizeState>>,
) -> Result<Json<Vec<PrizeDto>>, ApiError> {
    let prizes = state
        .service
        .get_all_prizes()
        .await
        .map_err(ApiError::from)?;
    Ok(Json(prizes))
}

pub async fn draw_prize_handler(
    State(state): State<Arc<PrizeState>>,
    Json(payload): Json<DrawPrizeRequestDto>,
) -> Result<Json<Vec<PrizeDrawDto>>, ApiError> {
    tracing::info!(
        target: "prize",
        "추첨 시작: prize_id={}, count={}",
        payload.prize_id,
        payload.count
    );

    let draws = state
        .prize_draw_service
        .draw_prize(payload)
        .await
        .map_err(ApiError::from)?;

    for draw in &draws {
        tracing::info!(
            target: "prize",
            "당첨: prize={}, user={}, ticket={}",
            draw.prize_name,
            draw.user_name,
            draw.ticket_number
        );
    }

    Ok(Json(draws))
}

pub async fn get_all_draws_handler(
    State(state): State<Arc<PrizeState>>,
) -> Result<Json<Vec<PrizeDrawDto>>, ApiError> {
    let draws = state
        .prize_draw_service
        .get_all_draws()
        .await
        .map_err(ApiError::from)?;
    Ok(Json(draws))
} 