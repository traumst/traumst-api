mod response;
mod router;
mod socket;

use router::handler;
use std::sync::Arc;
use log::{debug, error, info};
use tokio::net::{TcpListener};

use crate::{config, db, infra};

pub struct Server {
    db: Arc<db::pool::Bridge>,
}

impl Server {
    pub async fn new() -> Self {
        infra::logger::init();
        let options = db::pool::BridgeOptions {
            conn_str: config::db_conn_str(),
            pool_size: config::db_conn_pool(),
            op_timeout_ms: 5000, /* 5 sec */
        };
        let db = db::pool::Bridge::init(options).await;
        Self {
            db: Arc::new(db),
        }
    }

    pub async fn init(self) {
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
                        let shared_pool = self.db.clone();
                        debug!("  processing incoming request");
                        handler::handle_input(stream, shared_pool).await
                    }
                    Err(err) => error!("failed to read from socket; err: {err:?}"),
                }
            }
        });
    }
}