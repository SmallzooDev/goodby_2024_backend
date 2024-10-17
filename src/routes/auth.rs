use std::sync::Arc;
use crate::handler::auth_handler;
use crate::state::auth_state::AuthState;
use axum::{routing::post, Router};

pub fn routes() -> Router<Arc<AuthState>> {
    let router = Router::new().route("/auth", post(auth_handler::auth));
    return router;
}
