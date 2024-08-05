use std::net::SocketAddr;

use crate::{database, server::middle_ware};
use anyhow::Context;
use axum::{
    middleware::{self},
    routing::get,
    Router,
};
use leptos::LeptosOptions;
use leptos_axum::LeptosRoutes;
use sja_app;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

pub async fn setup_listener(addr: &SocketAddr) -> anyhow::Result<TcpListener> {
    TcpListener::bind(addr)
        .await
        .context("Error setting up TcpListener")
}

pub async fn setup_router(
    pool: Pool<Postgres>,
    leptos_options: LeptosOptions,
) -> anyhow::Result<Router> {
    let app_routes = leptos_axum::generate_route_list(sja_app::App);

    Ok(Router::new()
        .leptos_routes(&leptos_options, app_routes, sja_app::App)
        .with_state(leptos_options)
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
        .fallback(database::controller::fallback_handler))
}
