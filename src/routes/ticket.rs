use std::sync::Arc;
use axum::{routing::get, routing::post, Router};
use crate::handler::user_ticket_handler;
use crate::state::user_ticket_state::UserTicketState;

pub fn user_ticket_routes() -> Router<Arc<UserTicketState>> {
    Router::new()
        .route("/createTickets", post(user_ticket_handler::create_tickets_handler))
        .route("/ticketRanking", get(user_ticket_handler::get_ticket_ranking_handler))
}