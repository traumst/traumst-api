mod user;
mod email;
mod chat;

use super::RoutingResult;

pub async fn route(request: &str) -> RoutingResult {
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
        "chat" => send_options(),
        "user" => send_options(),
        "email" => send_options(),
        "ping" => send_options(),
        _ => RoutingResult::Err("404".to_string(), "Not Found".to_string(), path.to_string()),
    }
}

async fn get(path: &str) -> RoutingResult {
    let part = path.split('/').find(|&p| p.ne(""));
    match part.unwrap_or("not_found") {
        "ping" => send_pong(),
        _ => RoutingResult::Err("404".to_string(), "Not Found".to_string(), path.to_string()),
    }
}

async fn post(path: &str, request: &str) -> RoutingResult {
    let part = path.split('/').find(|&p| p.ne(""));
    match part.unwrap_or("not_found") {
        "chat" => post_chat(path, request).await,
        "user" => post_user(path, request).await,
        "email" => email::send(request),
        _ => RoutingResult::Err("404".to_string(), "Not Found".to_string(), path.to_string()),
    }
}

async fn post_user(path: &str, request: &str) -> RoutingResult {
    let part = path.split('/').rfind(|&p| p.ne(""));
    match part.unwrap_or("not_found") {
        "create" => user::create(request).await,
        "auth" => user::auth(request).await,
        _ => RoutingResult::Err("404".to_string(), "Not Found".to_string(), path.to_string()),
    }
}

async fn post_chat(path: &str, request: &str) -> RoutingResult {
    let part = path.split('/').rfind(|&p| p.ne(""));
    match part.unwrap_or("not_found") {
        "create" => chat::create(request).await,
        "join" => chat::join(request).await,
        "send" => chat::send(request).await,
        _ => RoutingResult::Err("404".to_string(), "Not Found".to_string(), path.to_string()),
    }
}

fn send_pong() -> RoutingResult { RoutingResult::Pong("PONG".to_string()) }

fn send_options() -> RoutingResult { RoutingResult::Options }