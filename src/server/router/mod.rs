pub mod handler;
mod routes;

use std::str::FromStr;
use log::{error};


const ACCESS_CONTROL_HEADERS: &str = r#"Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: Content-Type, Content-Length"#;

pub enum RoutingResult<'a> {
    User(&'a str, &'a str, u32),
    Email(&'a str, &'a str),
    Options(&'a str),
    Err(&'a str),
}

pub async fn handle_request(request: &str) -> RoutingResult {
    let parts: Vec<&str> = request.split_whitespace().collect();
    let method = parts[0];
    let path = parts[1];

    match method {
        "OPTIONS" => options(path),
        "GET" => get(path).await,
        "POST" => post(path, request).await,
        _ => RoutingResult::Err(request),
    }
}

fn options(path: &str) -> RoutingResult {
    match path {
        "/user" => send_options(),
        "/email" => send_options(),
        "/ping" => send_options(),
        _ => RoutingResult::Err(path),
    }
}

async fn get(path: &str) -> RoutingResult {
    match path {
        "/user" => send_user(path).await,
        "/ping" => send_pong(),
        _ => RoutingResult::Err(path),
    }
}

async fn post<'a>(path: &'a str, request: &'a str) -> RoutingResult<'a> {
    match path {
        "/user" => register_user(request).await,
        "/email" => send_email(request),
        _ => RoutingResult::Err(path),
    }
}

fn send_pong() -> RoutingResult<'static> {
    RoutingResult::Email(ACCESS_CONTROL_HEADERS, "pong")
}

fn send_options() -> RoutingResult<'static> {
    RoutingResult::Options(ACCESS_CONTROL_HEADERS)
}

async fn send_user(path: &str) -> RoutingResult {
    let args = path.split("\\");
    let user_id = args.last();
    if user_id.is_none() {
        error!("Missing mandatory argument");
        return RoutingResult::Err(path);
    }

    let user_id = u32::from_str(user_id.unwrap());
    if user_id.is_err() {
        error!("Invalid argument value");
        return RoutingResult::Err(path);
    }

    return RoutingResult::User(ACCESS_CONTROL_HEADERS, "", user_id.unwrap());
}

fn send_email(request: &str) -> RoutingResult {
    let mut headers_body = request.split("\r\n\r\n");
    let _headers = headers_body.next().expect("No headers were sent with request");
    match headers_body.next() {
        Some(body) => {
            RoutingResult::Email(ACCESS_CONTROL_HEADERS, body)
        }
        None => {
            error!("Body of email request appears empty");
            RoutingResult::Err(request)
        }
    }
}

async fn register_user(request: &str) -> RoutingResult {
    let mut headers_body = request.split("\r\n\r\n");
    let _headers = headers_body.next().expect("No headers were sent with request");
    match headers_body.next() {
        Some(body) => {
            RoutingResult::User(ACCESS_CONTROL_HEADERS, body, 0)
        }
        None => {
            error!("Body of email request appears empty");
            RoutingResult::Err(request)
        }
    }
}