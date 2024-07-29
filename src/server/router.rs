use super::middle_ware;
use crate::database;
use anyhow::Context;
use axum::{
    middleware::{self},
    routing::get,
    Extension, Router,
};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

pub async fn setup_listener(addr: &str, port: &str) -> anyhow::Result<TcpListener> {
    let listener_address = format!("{}:{}", addr, port);
    TcpListener::bind(listener_address)
        .await
        .context("error setting up tcplistener")
}

pub async fn setup_router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(|| async { "sja angebotverwaltung" }))
        .route("/db/angebote", get(database::services::get_angebote))
        .route(
            "/db/organisationen",
            get(database::services::get_organisationen),
        )
        .layer(Extension(pool))
        .layer(middleware::from_fn(middle_ware::logging_middleware))
}
