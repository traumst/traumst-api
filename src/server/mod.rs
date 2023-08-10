mod response;
mod router;

use std::sync::Arc;
use log::{debug, error, info, trace};
use sqlx::{Pool, Sqlite};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::{config, database, infra};

pub struct Server {
    db: Arc<Pool<Sqlite>>,
}

impl Server {
    pub async fn new() -> Self {
        infra::logger::init();
        Self {
            db: Arc::new(database::get_or_create().await),
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
                        let _shared_pool = self.db.clone();
                        debug!("  processing incoming request");
                        handle_input(stream).await
                    }
                    Err(err) => error!("failed to read from socket; err: {err:?}"),
                }
            }
        });
    }
}

pub async fn handle_input(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    match stream.read(&mut buffer).await {
        Ok(bytes_read) => match std::str::from_utf8(&buffer[..bytes_read]) {
            Ok(http_request) => process(stream, http_request).await,
            Err(e) => error!("Failed to read input into string: {e:?}"),
        }
        Err(e) => error!("Failed to read input stream {e:?}"),
    }
}

async fn process(mut stream: TcpStream, http_request: &str) {
    let routing_result = match router::handle_request(http_request) {
        router::RoutingResult::Email(head, body) => {
            send_email(head, body)
        }
        router::RoutingResult::Options(head) => Ok(response::Response {
            status_code: "204",
            status_message: "No Content",
            headers: head.to_string(),
            body: "".to_string(),
        }),
        router::RoutingResult::Err(path) => Err(format!("Routing failed to {path}"))
    };

    let result = translate(routing_result);
    match stream.write_all(result.as_bytes()).await {
        Ok(_) => trace!("Response output written"),
        Err(e) => error!("Failed to write response output {e:?}"),
    }
}

fn send_email(head: &str, body: &str) -> Result<response::Response, String> {
    match infra::email::parse_request(body) {
        Ok(json) => {
            match infra::email::send_email(json) {
                Ok(res) => Ok(response::Response {
                    status_code: "200",
                    status_message: "OK",
                    headers: head.to_string(),
                    body: res,
                }),
                Err(err) => Err(err)
            }
        }
        Err(msg) => { Err(msg) }
    }
}

fn translate(routing_result: Result<response::Response, String>) -> String {
    let result = match routing_result {
        Ok(routing) => response::Response {
            status_code: routing.status_code,
            status_message: routing.status_message,
            headers: routing.headers,
            body: routing.body
        },
        Err(error) => response::error(error),
    };

    response::generate_for(result)
}