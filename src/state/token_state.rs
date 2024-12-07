use crate::config::database::Database;
use crate::repository::user_repository::UserRepository;
use crate::service::token_service::{TokenService, TokenServiceTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct TokenState {
    pub token_service: TokenService,
    pub user_repo: Arc<UserRepository>,
}

impl TokenState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        let user_repo = Arc::new(UserRepository::new(db_conn));
        Self {
            token_service: TokenService::new(),
            user_repo,
        }
    }
}
