use std::sync::Arc;
use crate::error::{api_error::ApiError, token_error::TokenError, user_error::UserError};
use crate::repository::user_repository::UserRepositoryTrait;
use crate::service::token_service::TokenServiceTrait;
use crate::state::token_state::TokenState;
use axum::extract::State;
use axum::headers::authorization::{Authorization, Bearer};
use axum::headers::Header;
use axum::{http, http::Request, middleware::Next, response::IntoResponse};
use jsonwebtoken::errors::ErrorKind;

pub async fn admin<B>(
    State(state): State<Arc<TokenState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, ApiError> {
    let mut headers = req
        .headers_mut()
        .iter()
        .filter_map(|(header_name, header_value)| {
            if header_name == http::header::AUTHORIZATION {
                return Some(header_value);
            }
            None
        });

    let header: Authorization<Bearer> =
        Authorization::decode(&mut headers).map_err(|_| TokenError::MissingToken)?;
    let token = header.token();
    let token_state = Arc::clone(&state);
    match token_state.token_service.retrieve_token_claims(token) {
        Ok(token_data) => {
            let user = token_state.user_repo.find_by_name(token_data.claims.name).await;
            match user {
                Some(user) => {
                    if user.role == "admin" {
                        req.extensions_mut().insert(user);
                        Ok(next.run(req).await)
                    } else {
                        Err(UserError::Unauthorized)?
                    }
                }
                None => return Err(UserError::UserNotFound)?,
            }
        }
        Err(err) => {
            return match err.kind() {
                ErrorKind::ExpiredSignature => Err(TokenError::TokenExpired)?,
                _ => Err(TokenError::InvalidToken(token.parse().unwrap_or_default()))?,
            };
        }
    }
}