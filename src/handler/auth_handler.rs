use std::sync::Arc;
use crate::dto::{token_dto::TokenReadDto, user_dto::UserLoginDto};
use crate::error::{api_error::ApiError, request_error::ValidatedRequest, user_error::UserError};
use crate::repository::user_repository::UserRepositoryTrait;
use crate::service::token_service::TokenServiceTrait;
use crate::state::auth_state::AuthState;
use axum::{extract::State, Json};

pub async fn auth(
    State(state): State<Arc<AuthState>>,
    ValidatedRequest(payload): ValidatedRequest<UserLoginDto>,
) -> Result<Json<TokenReadDto>, ApiError> {
    let user = state
        .user_repo
        .find_by_name(payload.name)
        .await
        .ok_or(UserError::UserNotFound)?;

    return match state
        .user_service
        .verify_phone_number(&user, &payload.phone_number)
    {
        true => Ok(Json(state.token_service.generate_token(user)?)),
        false => Err(UserError::InvalidPassword)?,
    };
}