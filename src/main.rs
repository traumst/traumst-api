extern crate core;

mod router;
mod response;
mod request;
mod config;
mod email;
mod infra;

use std::io::ErrorKind;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use log::{debug, error, info, trace, warn};

const MIN_TIMEOUT: u64 = 250;
const MAX_TIMEOUT: u64 = 3500;

fn main() {
    let shutdown_signal = setup();

    let host = "0.0.0.0";
    let port = config::listen_on_port();
    let listen_on = format!("{}:{}", host, port);
    let listener = TcpListener::bind(listen_on.clone())
        .expect("Listener fail to bind");
    info!("server is listening on {listen_on:?}");

    listener.set_nonblocking(true).expect("Cannot set non-blocking on TcpListener");
    debug!("  listener set to non-blocking");

    let mut timeout = MIN_TIMEOUT;
    for stream in listener
        .incoming()
        .take_while(|_| !is_shutdown_requested(shutdown_signal.clone())){
        match stream {
            Ok(stream) => {
                trace!("  processing incoming request");
                request::handle_client(stream);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                if timeout < MAX_TIMEOUT { timeout *= 2; }
                trace!("  no incoming requests, sleeping {} millis", timeout);
                thread::sleep(Duration::from_millis(timeout.clone()));
            }
            Err(err) => {
                error!("TcpListener probably degraded, error: {err:?}");
                timeout = MAX_TIMEOUT;
            }
        }
    }
    warn!("Listener disconnected from {listen_on:?}")
}

fn setup() -> Arc<AtomicBool> {
    init_logger();
    setup_shutdown_handler()
}

fn init_logger() {
    infra::logger::init();
    error!("ERROR log enabled");
    warn!("WARN log enabled");
    info!("INFO log enabled");
    debug!("DEBUG log enabled");
    trace!("TRACE log enabled");
}

fn setup_shutdown_handler() -> Arc<AtomicBool> {
    let shutdown = Arc::new(AtomicBool::new(false));
    let server_shutdown = shutdown.clone();
    ctrlc::set_handler(move || {
        error!("Shutdown signal received");
        server_shutdown.store(true, Ordering::Relaxed);
    }).expect("error setting Ctrl-C handler");
    shutdown
}

fn is_shutdown_requested(shutdown_requested: Arc<AtomicBool>) -> bool {
    shutdown_requested.load(Ordering::Relaxed)
}