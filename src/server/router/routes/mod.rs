mod email;
mod user;

use super::RoutingResult;
use super::ACCESS_CONTROL_HEADERS;

pub use email::send_email;
pub use user::get_user;
pub use user::create_user;

pub async fn handle_request(request: &str) -> RoutingResult {
    let parts: Vec<&str> = request.split_whitespace().collect();
    let method = parts[0];
    let path = parts[1];

    match method {
        "OPTIONS" => options(path),
        "GET" => get(path).await,
        "POST" => post(path, request).await,
        _ => RoutingResult::Err("405".to_string(), "Method Not Allowed".to_string(), path.to_string()),
    }
}

fn options(path: &str) -> RoutingResult {
    let part = path.split('/').find(|&p| p.ne(""));
    match part.unwrap_or("not_found") {
        "user" => send_options(),
        "email" => send_options(),
        "ping" => send_options(),
        _ => RoutingResult::Err("404".to_string(), "Not Found".to_string(), path.to_string()),
    }
}

async fn get(path: &str) -> RoutingResult {
    let part = path.split('/').find(|&p| p.ne(""));
    match part.unwrap_or("not_found") {
        "user" => super::user::get(path).await,
        "ping" => send_pong(),
        _ => RoutingResult::Err("404".to_string(), "Not Found".to_string(), path.to_string()),
    }
}

async fn post(path: &str, request: &str) -> RoutingResult {
    let part = path.split('/').find(|&p| p.ne(""));
    match part.unwrap_or("not_found") {
        "user" => super::user::create(request).await,
        "email" => super::email::send_email(request),
        _ => RoutingResult::Err("404".to_string(), "Not Found".to_string(), path.to_string()),
    }
}

fn send_pong() -> RoutingResult { RoutingResult::Pong(ACCESS_CONTROL_HEADERS.to_string(), "PONG".to_string()) }

fn send_options() -> RoutingResult { RoutingResult::Options(ACCESS_CONTROL_HEADERS.to_string()) }