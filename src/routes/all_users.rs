use crate::handler::users_handler;
use crate::state::user_state::UserState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn routes() -> Router<Arc<UserState>> {
    Router::new().route("/allUsers", get(users_handler::get_all_users))
}
