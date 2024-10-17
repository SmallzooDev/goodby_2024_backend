use std::sync::Arc;
use crate::dto::user_dto::UserReadDto;
use crate::error::api_error::ApiError;
use crate::state::user_state::UserState;
use axum::{extract::State, Json};

pub async fn get_all_users(
    State(state): State<Arc<UserState>>,
) -> Result<Json<Vec<UserReadDto>>, ApiError> {
    let users = state
        .user_service
        .get_all_users()
        .await
        .map_err(ApiError::from)?;
    Ok(Json(users))
}
