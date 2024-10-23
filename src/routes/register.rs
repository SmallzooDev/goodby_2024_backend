use crate::handler::register_handler;
use crate::state::user_state::UserState;
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn routes() -> Router<Arc<UserState>> {
    Router::new().route("/register", post(register_handler::register))
}
