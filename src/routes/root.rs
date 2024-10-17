use super::{all_users, auth};
use crate::config::database::Database;
use crate::middleware::auth as auth_middleware;
use crate::middleware::admin as admin_middleware;
use crate::routes::{profile, register};
use crate::routes::ticket::user_ticket_routes;
use crate::state::{auth_state::AuthState, token_state::TokenState, user_state::UserState, user_ticket_state::UserTicketState};
use axum::routing::{get, IntoMakeService};
use axum::{middleware, Router};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn routes(db_conn: Arc<Database>) -> IntoMakeService<Router> {
    let auth_state = Arc::new(AuthState::new(&db_conn));
    let user_state = Arc::new(UserState::new(&db_conn));
    let token_state = Arc::new(TokenState::new(&db_conn));
    let user_ticket_state = Arc::new(UserTicketState::new(&db_conn));

    let public_routes = Router::new()
        .merge(auth::routes().with_state(auth_state.clone()))
        .merge(register::routes().with_state(user_state.clone()))
        .route("/health", get(|| async { "Healthy..." }));

    let private_routes = Router::new()
        .merge(profile::routes()
            .layer(ServiceBuilder::new().layer(
                middleware::from_fn_with_state(token_state.clone(), auth_middleware::auth),
            )));

    let admin_routes = Router::new()
        .merge(all_users::routes().with_state(user_state.clone()))
        .merge(user_ticket_routes().with_state(user_ticket_state.clone()))
        .layer(ServiceBuilder::new().layer(
            middleware::from_fn_with_state(token_state.clone(), admin_middleware::admin),
        ));

    let app_router = Router::new()
        .nest("/api", public_routes)
        .nest("/api/private", private_routes)
        .nest("/api/admin", admin_routes)
        .layer(TraceLayer::new_for_http());

    app_router.into_make_service()
}