mod structs;

use anyhow::Context;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("Error loading dotenv")?;
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL should be set")?;
    let pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .context("Database pool could not be created")?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Error running the migrations")?;

    Ok(())
}
