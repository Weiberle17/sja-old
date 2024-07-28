use anyhow::Context;
use axum::{
    body::Body,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use chrono::Local;
use tokio::net::TcpListener;

pub async fn setup_listener(addr: &str, port: &str) -> anyhow::Result<TcpListener> {
    let listener_address = format!("{}:{}", addr, port);
    TcpListener::bind(listener_address)
        .await
        .context("Error setting up TcpListener")
}

pub async fn setup_router() -> Router {
    Router::new()
        .route("/", get(|| async { "SJA Angebotverwaltung" }))
        .layer(middleware::from_fn(logging_middleware))
}

async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
    println!("Received a request to {} at: {}", req.uri(), Local::now());
    next.run(req).await
}
