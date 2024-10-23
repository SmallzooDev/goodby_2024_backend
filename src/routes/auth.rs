use crate::handler::auth_handler;
use crate::state::auth_state::AuthState;
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn routes() -> Router<Arc<AuthState>> {
    Router::new().route("/auth", post(auth_handler::auth))
}
