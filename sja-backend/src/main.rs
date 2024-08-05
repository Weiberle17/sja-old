use anyhow::Context;
use sja_backend::{database, server};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).context("couldn't initialize logging")?;

    dotenvy::dotenv().context("Error loading dotenv")?;
    let database_url: String = env::var("DATABASE_URL").context("DATABASE_URL should be set")?;
    let conf = leptos::get_configuration(None)
        .await
        .context("Error getting Leptos config")?;
    let leptos_options = conf.leptos_options;
    let server_url = leptos_options.site_addr;
    let pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .context("Database pool could not be created")?;

    database::controller::migrations(&pool).await?;

    let router = server::router::setup_router(pool, leptos_options).await?;
    let listener = server::router::setup_listener(&server_url).await?;
    log::info!("Running on {}", &server_url);
    axum::serve(listener, router)
        .await
        .context("Error serving backend")?;

    Ok(())
}
