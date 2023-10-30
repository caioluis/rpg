use anyhow::{
    Context,
    Result,
};

use api::router::Controller;

#[tokio::main]
async fn main() -> Result<()> {
    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&db_connection_str)
        .await
        .expect("Failed to connect to Postgres");

    Controller::serve(&pool).await.context("Error while serving the API")?;
    Ok(())
}