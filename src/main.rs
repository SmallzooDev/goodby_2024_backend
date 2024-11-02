use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use crate::config::{database, parameter};
use crate::config::database::DatabaseTrait;
use tracing::{info, Level};

mod config;
mod routes;
mod dto;
mod error;
mod response;
mod entity;
mod repository;
mod state;
mod service;
mod middleware;
mod handler;

#[tokio::main]
async fn main() {
    parameter::init();
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let connection = database::Database::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let app = routes::root::routes(Arc::new(connection)).layer(cors);

    let port = parameter::get("PORT");
    let host = format!("0.0.0.0:{}", port);
    info!("Server is running on port {}", port);

    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e.to_string()));
}