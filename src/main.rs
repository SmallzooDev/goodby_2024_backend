use std::sync::Arc;
use tower_http::cors::{CorsLayer};
use crate::config::{database, parameter};
use crate::config::database::DatabaseTrait;
use tracing::info;
use std::time::Duration;
use axum::http::header;
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
    config::parameter::init();
    config::logging::setup_logging().expect("Failed to setup logging");
    info!("Starting Goodbye 2024 Backend...");

    let connection = database::Database::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {}", e.to_string()));
    info!("Database connected successfully");

    let cors = CorsLayer::new()
        .allow_headers([
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::OPTIONS,
        ])
        .allow_origin(["http://localhost:3000".parse().unwrap(), "http://localhost:5173".parse().unwrap()])
        .allow_credentials(true)
        .expose_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
        ])
        .max_age(Duration::from_secs(60 * 60));

    let app = routes::root::routes(Arc::new(connection))
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(middleware::logging::log_request));

    let port = parameter::get("PORT");
    let host = format!("0.0.0.0:{}", port);
    info!("Server is running on port {}", port);

    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Server error: {}", e);
            panic!("Server error: {}", e.to_string())
        });
}