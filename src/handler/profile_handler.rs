use crate::dto::user_dto::{StatusResponse, UserMeDto};
use crate::entity::user::User;
use crate::error::api_error::ApiError;
use crate::state::user_state::UserState;
use axum::{Extension, Json, extract::State};
use std::sync::Arc;

pub async fn profile(
    Extension(current_user): Extension<User>,
    State(state): State<Arc<UserState>>,
) -> Result<Json<UserMeDto>, ApiError> {
    let user_details = state.user_service.get_user_details(current_user.id).await?;
    Ok(Json(user_details))
}

pub async fn me(
    Extension(_current_user): Extension<User>,
) -> Result<Json<StatusResponse>, ApiError> {
    Ok(Json(StatusResponse {
        message: "ok".to_string(),
        code: 200,
    }))
} 