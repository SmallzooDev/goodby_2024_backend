use std::sync::Arc;
use crate::handler::users_handler;
use crate::state::user_state::UserState;
use axum::{routing::get, Router};

pub fn routes() -> Router<Arc<UserState>> {
    let router = Router::new().route("/allUsers", get(users_handler::get_all_users));
    router
}
