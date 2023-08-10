mod config;
mod infra;
mod server;
mod database;

use std::error;
use log::{error, warn};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let srv = server::Server::new().await;
    srv.init().await;

    match signal::ctrl_c().await {
        Ok(()) => warn!("Shutdown signal received"),
        Err(err) => error!("Unable to listen for shutdown signal: {err:?}")
    }

    Ok(warn!("Application will exit"))
}