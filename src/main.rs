mod router;
mod structs;

use anyhow::Context;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("Error loading dotenv")?;
    let database_url: String = env::var("DATABASE_URL").context("DATABASE_URL should be set")?;
    let server_url: String = env::var("SERVER_URL").context("SERVER_URL should be set")?;
    let server_port: String = env::var("SERVER_PORT").context("SERVER_PORT should be set")?;
    let pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .context("Database pool could not be created")?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Error running the migrations")?;

    let router = router::setup_router().await;
    let listener = router::setup_listener(&server_url, &server_port).await?;
    axum::serve(listener, router)
        .await
        .context("Error serving backend")?;
    println!("Running on {}:{}", &server_url, &server_port);

    Ok(())
}
