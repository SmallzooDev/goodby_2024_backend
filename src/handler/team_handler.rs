use crate::dto::team_request_dto::{TeamAssignRequestDto, TeamCreateRequestDto, TeamUserDto};
use crate::error::api_error::ApiError;
use crate::state::team_state::TeamState;
use axum::{extract::State, Json};
use std::sync::Arc;

pub async fn create_team(
    State(state): State<Arc<TeamState>>,
    Json(payload): Json<TeamCreateRequestDto>,
) -> Result<Json<i32>, ApiError> {
    let team_id = state.team_service.create_team(payload).await?;
    Ok(Json(team_id))
}

pub async fn assign_team(
    State(state): State<Arc<TeamState>>,
    Json(payload): Json<TeamAssignRequestDto>,
) -> Result<Json<()>, ApiError> {
    state.team_service.assign_team(payload).await?;
    Ok(Json(()))
}

pub async fn get_team_users(
    State(state): State<Arc<TeamState>>,
) -> Result<Json<Vec<TeamUserDto>>, ApiError> {
    let team_users = state.team_service.get_team_users().await?;
    Ok(Json(team_users))
} 