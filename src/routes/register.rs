use std::sync::Arc;
use crate::handler::register_handler;
use crate::state::user_state::UserState;
use axum::{routing::post, Router};

pub fn routes() -> Router<Arc<UserState>> {
    let router = Router::new().route("/register", post(register_handler::register));
    router
}
