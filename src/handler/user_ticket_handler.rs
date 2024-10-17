use crate::dto::ticket_creation_result::TicketCreationResult;
use crate::dto::user_ticket_count::UserTicketCount;
use crate::error::api_error::ApiError;
use crate::state::user_ticket_state::UserTicketState;
use axum::{extract::State, Json};
use std::sync::Arc;
use crate::dto::user_ids_request_dto::UserIdsRequestDto;

pub async fn create_tickets_handler(
    State(state): State<Arc<UserTicketState>>,
    Json(payload): Json<UserIdsRequestDto>,
) -> Result<Json<Vec<TicketCreationResult>>, ApiError> {
    let results = state
        .user_ticket_service
        .create_tickets_for_users(payload.users_id)
        .await
        .map_err(ApiError::from)?;
    Ok(Json(results))
}

pub async fn get_ticket_ranking_handler(
    State(state): State<Arc<UserTicketState>>,
) -> Result<Json<Vec<UserTicketCount>>, ApiError> {
    let rankings = state
        .user_ticket_service
        .get_ticket_ranking()
        .await
        .map_err(ApiError::from)?;
    Ok(Json(rankings))
}