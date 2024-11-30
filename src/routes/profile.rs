use crate::handler::profile_handler;
use crate::state::user_state::UserState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn routes() -> Router<Arc<UserState>> {
    Router::new()
        .route("/profile", get(profile_handler::profile))
        .route("/me", get(profile_handler::me))
} 