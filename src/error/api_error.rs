use crate::error::{db_error::DbError, token_error::TokenError, user_error::UserError};
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Token(#[from] TokenError),

    #[error(transparent)]
    User(#[from] UserError),

    #[error(transparent)]
    Db(#[from] DbError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Token(error) => error.into_response(),
            ApiError::User(error) => error.into_response(),
            ApiError::Db(error) => error.into_response(),
        }
    }
}
