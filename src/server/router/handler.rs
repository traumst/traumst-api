use std::sync::Arc;
use log::{error, trace};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::server::response;
use crate::server::router;
use crate::db;

use super::routes;

pub async fn process(mut stream: TcpStream, http_request: &str, shared_pool: Arc<db::pool::Bridge>) {
    let routed = route(http_request, shared_pool).await;
    let result = translate(routed);
    match stream.write_all(result.as_bytes()).await {
        Ok(_) => trace!("Response output written"),
        Err(e) => error!("Failed to write response output {e:?}"),
    }
}

async fn route(http_request: &str, shared_pool: Arc<db::pool::Bridge>) -> Result<response::Response, response::Response> {
    let res = routes::handle_request(http_request).await;
    match res {
        router::RoutingResult::User(head, body, user_id) => {
            if user_id == 0 {
                routes::create_user(head.as_str(), body.as_str(), shared_pool).await
            } else {
                routes::get_user(head.as_str(), user_id, shared_pool).await
            }
        }
        router::RoutingResult::Email(head, body) => {
            routes::send_email(head.as_str(), body.as_str())
        }
        router::RoutingResult::Options(head) => Ok(response::Response {
            status_code: "204".to_string(),
            status_message: "No Content".to_string(),
            headers: head.to_string(),
            body: "".to_string(),
        }),
        router::RoutingResult::Pong(head, body) => Ok(response::Response {
            status_code: "200".to_string(),
            status_message: "Ok".to_string(),
            headers: head.to_string(),
            body: body.to_string(),
        }),
        router::RoutingResult::Err(code, status, body) => Err(response::Response {
            status_code: code,
            status_message: status,
            headers: "".to_string(),
            body: body.to_string(),
        })
    }
}

fn translate(routing_result: Result<response::Response, response::Response>) -> String {
    let result = match routing_result {
        Ok(routing) => response::Response {
            status_code: routing.status_code,
            status_message: routing.status_message,
            headers: routing.headers,
            body: routing.body
        },
        Err(error) => {
            response::error(error.status_message.as_str(), error.body)
        },
    };

    response::generate_for(result)
}