use std::sync::Arc;
use log::error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use crate::db::pool::Bridge;

mod handler;
mod routes;
mod user;
mod email;

pub const ACCESS_CONTROL_HEADERS: &str = r#"Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: Content-Type, Content-Length"#;

pub enum RoutingResult {
    User(String, String, u32),
    Email(String, String),
    Pong(String, String),
    Options(String),
    Err(String, String, String),
}

pub async fn handle_input(mut stream: TcpStream, shared_pool: Arc<Bridge>) {
    let mut buffer = [0; 2048];
    match stream.read(&mut buffer).await {
        Ok(bytes_read) => match std::str::from_utf8(&buffer[..bytes_read]) {
            Ok(http_request) => handler::process(stream, http_request, shared_pool).await,
            Err(e) => error!("Failed to read input into string: {e:?}"),
        }
        Err(e) => error!("Failed to read input stream {e:?}"),
    }
}