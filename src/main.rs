extern crate core;

mod router;
mod response;
mod request;
mod config;
mod email;
mod infra;
mod database;

use std::error;
use log::{debug, error, info, trace, warn};
use sqlx::{Pool, Sqlite};
use tokio::net::TcpListener;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut db_pool = setup();

    let host = "0.0.0.0";
    let port = config::listen_on_port();
    let listen_on = format!("{}:{}", host, port);
    let listener = TcpListener::bind(listen_on.clone())
        .await
        .expect("Listener fail to bind");
    info!("Server is listening on {listen_on:?}");

    tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    debug!("  processing incoming request");
                    request::handle_client(stream).await
                }
                Err(err) => error!("failed to read from socket; err: {err:?}"),
            }
        }
    });

    match signal::ctrl_c().await {
        Ok(()) => warn!("Shutdown signal received"),
        Err(err) => error!("Unable to listen for shutdown signal: {err:?}")
    }

    Ok(warn!("Listener disconnected from {listen_on:?}"))
}

async fn setup() -> Pool<Sqlite> {
    init_logger();
    database::create().await
}

fn init_logger() {
    infra::logger::init();
    error!("ERROR log enabled");
    warn!("WARN log enabled");
    info!("INFO log enabled");
    debug!("DEBUG log enabled");
    trace!("TRACE log enabled");
}