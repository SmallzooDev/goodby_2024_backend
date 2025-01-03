use crate::handler::prize_handler;
use crate::state::prize_state::PrizeState;
use axum::{routing::get, routing::post, Router};
use std::sync::Arc;

pub fn admin_prize_routes() -> Router<Arc<PrizeState>> {
    Router::new()
        .route("/prize/create", post(prize_handler::create_prize_handler))
        .route("/prize/draw", post(prize_handler::draw_prize_handler))
}

pub fn public_prize_routes() -> Router<Arc<PrizeState>> {
    Router::new()
        .route("/prize/list", get(prize_handler::get_prizes_handler))
        .route("/prize/draws", get(prize_handler::get_all_draws_handler))
} 