mod api;
mod response;
mod socket;

use log::trace;
use log::debug;
use log::info;
use log::error;
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::db;
use crate::config;
use crate::infra;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use response::generate_error;
use response::generate_for;
use response::Response;

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
                        tokio::spawn(async move {
                            debug!("  processing incoming request");
                            handle_input(stream, shared_pool).await
                        });
                    }
                    Err(err) => error!("failed to read from socket; err: {err:?}"),
                }
            }
        });
    }
}

async fn handle_input(mut stream: TcpStream, shared_pool: Arc<db::pool::Bridge>) {
    let mut buffer = [0; 2048];
    match stream.read(&mut buffer).await {
        Ok(bytes_read) => match std::str::from_utf8(&buffer[..bytes_read]) {
            Ok(http_request) => process(stream, http_request, shared_pool).await,
            Err(e) => error!("Failed to read input into string: {e:?}"),
        }
        Err(e) => error!("Failed to read input stream {e:?}"),
    }
}

async fn process(mut stream: TcpStream, http_request: &str, shared_pool: Arc<db::pool::Bridge>) {
    let routed = api::route(http_request, shared_pool).await;
    let result = translate(routed);
    match stream.write_all(result.as_bytes()).await {
        Ok(_) => trace!("Response output written"),
        Err(e) => error!("Failed to write response output {e:?}"),
    }
}

fn translate(routing_result: Result<Response, Response>) -> String {
    let result = match routing_result {
        Ok(routing) => Response {
            status_code: routing.status_code,
            status_message: routing.status_message,
            headers: routing.headers,
            body: routing.body
        },
        Err(error) => {
            generate_error(error.status_message.as_str(), error.body)
        }
    };
    generate_for(result)
}