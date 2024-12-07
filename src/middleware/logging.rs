use axum::{
    middleware::Next,
    response::Response,
    http::Request,
};
use std::time::Instant;
use tracing::Instrument;

pub async fn log_request<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    let span = tracing::info_span!(
        "request",
        method = %method,
        uri = %uri,
        status = tracing::field::Empty,
        latency = tracing::field::Empty,
    );
    let span_clone = span.clone();

    async move {
        let response = next.run(request).await;
        
        let latency = start.elapsed();
        span.record("status", &tracing::field::display(response.status()));
        span.record("latency", &tracing::field::display(format!("{:?}", latency)));
        
        tracing::info!(parent: &span, "Request completed");
        
        response
    }.instrument(span_clone).await
} 