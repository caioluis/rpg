use axum;
use axum::{middleware, Router};
use anyhow::{
    Context,
    Result
};

use std::{
    net::SocketAddr,
};
use crate::auth_middleware;

use crate::endpoints::{
    banking::BankingRouter,
    core::CoreRouter
};

pub struct Controller;

impl Controller {
    pub async fn serve(pool: &sqlx::PgPool) -> Result<()> {
        let router = Router::new()
            .nest("/banking", BankingRouter::new_router(pool.clone()).clone())
            .nest("/core", CoreRouter::new_router(pool.clone()).clone())
            .route("/", axum::routing::get(|| async { "Hello, world!" }))
            .route_layer(middleware::from_fn(auth_middleware::auth));

        // PORT or 3000
        let port_env = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
        let port = port_env.parse::<u16>().expect("Couldn't parse PORT");

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        println!("Listening prod on {}", addr);

        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .context("Error while serving the API")?;

        Ok(())
    }
}