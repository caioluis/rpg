use anyhow::{
    Context,
    Result,
};
use tokio::time::{sleep};
use api::router::Controller;

#[tokio::main]
async fn main() -> Result<()> {
    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut retries = 0;
    let max_retries = 5;  // Set your max retries

    let pool = loop {
        match sqlx::postgres::PgPoolOptions::new()
            .max_connections(50)
            .acquire_timeout(std::time::Duration::from_secs(30))
            .connect(&db_connection_str)
            .await
        {
            Ok(pool) => break pool,
            Err(_e) if retries < max_retries => {
                retries += 1;
                eprintln!("Failed to connect to Postgres, retrying... ({} retries left)", max_retries - retries);
                sleep(std::time::Duration::from_secs(5)).await;
            },
            Err(e) => return Err(e.into()),
        }
    };

    Controller::serve(&pool).await.context("Error while serving the API")?;
    Ok(())
}