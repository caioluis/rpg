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

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        println!("Listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .context("Error while serving the API")?;

        Ok(())
    }
}