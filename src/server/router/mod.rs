use log::{error};

const ACCESS_CONTROL_HEADERS: &str = r#"Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: Content-Type, Content-Length"#;

pub enum RoutingResult<'a> {
    Email(&'a str, &'a str),
    Options(&'a str),
    Err(&'a str),
}

pub fn handle_request(request: &str) -> RoutingResult {
    let parts: Vec<&str> = request.split_whitespace().collect();
    let method = parts[0];
    let path = parts[1];

    match method {
        "OPTIONS" => options(path),
        "GET" => get(path),
        "POST" => post(path, request),
        _ => RoutingResult::Err(request),
    }
}

fn options(path: &str) -> RoutingResult {
    match path {
        "/email" => send_options(),
        "/ping" => send_options(),
        _ => RoutingResult::Err(path),
    }
}

fn get(path: &str) -> RoutingResult {
    match path {
        "/ping" => send_pong(),
        _ => RoutingResult::Err(path),
    }
}

fn post<'a>(path: &'a str, request: &'a str) -> RoutingResult<'a> {
    match path {
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