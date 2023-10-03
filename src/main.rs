mod db;
mod rest;
mod view;

use crate::db::init_db;
use anyhow::Result;
use axum::{Extension, Router};
use sqlx::SqlitePool;
use std::net::SocketAddr;

/// Build the overall web service router.
/// Constructing the router in a function makes it easy to re-use in unit tests.
fn router(connection_pool: SqlitePool) -> Router {
    Router::new()
        // Nest service allows you to attach another router to a URL base.
        // "/" inside the service will be "/books" to the outside world.
        .nest_service("/books", rest::books_service())
        // Add the web view
        .nest_service("/", view::view_service())
        // Add the connection pool as a "layer", available for dependency injection.
        .layer(Extension(connection_pool))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env if available
    dotenv::dotenv().ok();

    // Initialize the database and obtain a connection pool
    let connection_pool = init_db().await?;

    // Initialize the Axum routing service
    let app = router(connection_pool);

    // Define the address to listen on (everything)
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
