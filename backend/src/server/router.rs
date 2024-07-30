use super::middle_ware;
use crate::database;
use anyhow::Context;
use axum::{
    middleware::{self},
    routing::get,
    Router,
};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

pub async fn setup_listener(addr: &str, port: &str) -> anyhow::Result<TcpListener> {
    let listener_address = format!("{}:{}", addr, port);
    TcpListener::bind(listener_address)
        .await
        .context("Error setting up TcpListener")
}

pub async fn setup_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(|| async { "SJA Angebotverwaltung" }))
        .route("/api/db/angebote", get(database::controller::get_angebote))
        // .route(
        //     "/api/db/organisationen",
        //     get(database::services::get_organisationen),
        // )
        // .route(
        //     "/api/db/ansprechpartner",
        //     get(database::services::get_ansprechpartner),
        // )
        .route("/api/db/links", get(database::controller::get_links))
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(middleware::from_fn(middle_ware::logging_middleware))
        .fallback(database::controller::fallback_handler)
}
