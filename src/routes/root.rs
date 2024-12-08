use super::{all_users, auth, prize};
use crate::config::database::Database;
use crate::middleware::admin as admin_middleware;
use crate::middleware::auth as auth_middleware;
use crate::routes::ticket::user_ticket_routes;
use crate::routes::{ register, team, profile};
use crate::state::{auth_state::AuthState, token_state::TokenState, user_state::UserState, user_ticket_state::UserTicketState, team_state::TeamState};
use axum::routing::get;
use axum::{middleware, Router};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any};
use crate::state::prize_state::PrizeState;

pub async fn create_routes(db_conn: Arc<Database>) -> Router {
    let auth_state = Arc::new(AuthState::new(&db_conn));
    let user_state = Arc::new(UserState::new(&db_conn));
    let token_state = Arc::new(TokenState::new(&db_conn));
    let user_ticket_state = Arc::new(UserTicketState::new(db_conn.clone()));
    let team_state = Arc::new(TeamState::new(&db_conn));
    let prize_state = Arc::new(PrizeState::new(db_conn.clone()));

    let public_routes = Router::new()
        .merge(auth::routes().with_state(auth_state.clone()))
        .merge(register::routes().with_state(user_state.clone()))
        .merge(prize::public_prize_routes().with_state(prize_state.clone()))
        .route("/health", get(|| async { "Healthy..." }));

    let private_routes = Router::new()
        .merge(profile::routes().with_state(user_state.clone()))
        .layer(ServiceBuilder::new().layer(
            middleware::from_fn_with_state(token_state.clone(), auth_middleware::auth),
        ));

    let admin_routes = Router::new()
        .merge(all_users::routes().with_state(user_state.clone()))
        .merge(user_ticket_routes().with_state(user_ticket_state.clone()))
        .merge(team::team_routes().with_state(team_state.clone()))
        .merge(prize::admin_prize_routes().with_state(prize_state.clone()))
        .layer(ServiceBuilder::new().layer(
            middleware::from_fn_with_state(token_state.clone(), admin_middleware::admin),
        ));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app_router = Router::new()
        .nest("/api", public_routes)
        .nest("/api/private", private_routes)
        .nest("/api/admin", admin_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    app_router
}