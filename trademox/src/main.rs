use crate::app::app_box::build_app;
use crate::logging::tracing_initial::init_tracing;
use crate::websocket::ws_router::create_socket_router;
use anyhow::Result;
use dotenv::dotenv;
use std::env::var;
use std::error::Error;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;

mod app;
mod configs;
mod creator;
mod data;
mod errors;
mod logging;
mod requests;
pub mod websocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_tracing();

    dotenv().ok();

    let app = build_app();
    let router = create_socket_router(app);

    let host = var("HOST")?;
    let port = var("PORT")?;
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&addr).await?;
    info!("Server listening on {}:{}", host, port);
    let _ = axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
    info!("Shutdown signal received");
}
