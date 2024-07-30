use crate::database::services;
use anyhow::Context;
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use sqlx::{Pool, Postgres};

pub async fn migrations(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("Error running the migrations")?;

    Ok(())
}

pub async fn get_angebote(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    let angebote = match services::select_all_angebote(&pool).await {
        Ok(angebote) => angebote,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error fetching all Angebote: {}", e),
            )
                .into_response()
        }
    };
    (StatusCode::OK, Json(angebote)).into_response()
}

pub async fn get_links(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    let links = match services::select_links(&pool).await {
        Ok(links) => links,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error fetching all Links: {}", e),
            )
                .into_response()
        }
    };

    (StatusCode::OK, Json(links)).into_response()
}

pub async fn fallback_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Resource not found").into_response()
}
