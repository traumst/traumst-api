use std::sync::Arc;
use log::{error, trace};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::db::pool::Bridge;
use crate::server::response;
use crate::server::router;
use crate::server::router::routes;

pub async fn handle_input(mut stream: TcpStream, shared_pool: Arc<Bridge>) {
    let mut buffer = [0; 2048];
    match stream.read(&mut buffer).await {
        Ok(bytes_read) => match std::str::from_utf8(&buffer[..bytes_read]) {
            Ok(http_request) => process(stream, http_request, shared_pool).await,
            Err(e) => error!("Failed to read input into string: {e:?}"),
        }
        Err(e) => error!("Failed to read input stream {e:?}"),
    }
}

async fn process(mut stream: TcpStream, http_request: &str, shared_pool: Arc<Bridge>) {
    let routed = route(http_request, shared_pool).await;
    let result = translate(routed);
    match stream.write_all(result.as_bytes()).await {
        Ok(_) => trace!("Response output written"),
        Err(e) => error!("Failed to write response output {e:?}"),
    }
}

async fn route(http_request: &str, shared_pool: Arc<Bridge>) -> Result<response::Response, String> {
    match router::handle_request(http_request).await {
        router::RoutingResult::User(head, body, user_id) => {
            if user_id == 0 {
                routes::create_user(head, body, shared_pool).await
            } else {
                routes::get_user(head, user_id, shared_pool).await
            }
        }
        router::RoutingResult::Email(head, body) => {
            routes::send_email(head, body)
        }
        router::RoutingResult::Options(head) => Ok(response::Response {
            status_code: "204",
            status_message: "No Content",
            headers: head.to_string(),
            body: "".to_string(),
        }),
        router::RoutingResult::Pong(head, body) => Ok(response::Response {
            status_code: "200",
            status_message: "Ok",
            headers: head.to_string(),
            body: body.to_string(),
        }),
        router::RoutingResult::Err(path) => Err(format!("Routing failed to {path}"))
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