use crate::database::structs::{Angebot, Ansprechpartner, Organisation};
use anyhow::Context;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::{Pool, Postgres};

pub async fn migrations(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("Error running the migrations")?;

    Ok(())
}

pub async fn get_angebote(Extension(pool): Extension<Pool<Postgres>>) -> impl IntoResponse {
    let angebote = match sqlx::query_as!(
        Angebot,
        r#"
        SELECT
          *
        FROM
          angebot;
        "#
    )
    .fetch_all(&pool)
    .await
    {
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

pub async fn get_organisationen(Extension(pool): Extension<Pool<Postgres>>) -> impl IntoResponse {
    let organisationen = match sqlx::query_as!(
        Organisation,
        r#"
        SELECT
          *
        FROM
          organisation;
        "#
    )
    .fetch_all(&pool)
    .await
    {
        Ok(organisationen) => organisationen,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error fetching all Organisationen: {}", e),
            )
                .into_response()
        }
    };

    (StatusCode::OK, Json(organisationen)).into_response()
}

pub async fn get_ansprechpartner(Extension(pool): Extension<Pool<Postgres>>) -> impl IntoResponse {
    let ansprechpartner = match sqlx::query_as!(
        Ansprechpartner,
        r#"
        SELECT
          *
        FROM
          ansprech_partner;
        "#
    )
    .fetch_all(&pool)
    .await
    {
        Ok(ansprechpartner) => ansprechpartner,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error fetching all Ansprechpartner: {}", e),
            )
                .into_response()
        }
    };

    (StatusCode::OK, Json(ansprechpartner)).into_response()
}
