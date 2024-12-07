use crate::config::database::Database;
use crate::repository::user_repository::UserRepository;
use crate::service::user_service::UserService;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserState {
    pub user_service: UserService,
}

impl UserState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        let user_repo = Arc::new(UserRepository::new(db_conn));
        Self {
            user_service: UserService::new(user_repo),
        }
    }
}
