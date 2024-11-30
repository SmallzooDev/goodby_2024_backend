use crate::handler::team_handler;
use crate::state::team_state::TeamState;
use axum::{ routing::get, routing::post, Router};
use std::sync::Arc;

pub fn team_routes() -> Router<Arc<TeamState>> {
    Router::new()
        .route("/team/create", post(team_handler::create_team))
        .route("/team/assign", post(team_handler::assign_team))
        .route("/team/users", get(team_handler::get_team_users))
} 