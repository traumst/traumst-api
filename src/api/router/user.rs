use log::error;
use crate::api::Action;
use super::RoutingResult;
use super::ACCESS_CONTROL_HEADERS;

pub async fn create(request: &str) -> RoutingResult {
    let mut headers_body = request.split("\r\n\r\n");
    let _headers = headers_body.next().expect("No headers were sent with request");
    match headers_body.next() {
        Some(body) => {
            RoutingResult::User(Action::Create, ACCESS_CONTROL_HEADERS.to_string(), body.to_string())
        }
        None => {
            error!("Body of email request appears empty");
            return RoutingResult::Err("400".to_string(), "Bad Request".to_string(), request.to_string());
        }
    }
}

pub async fn auth(request: &str) -> RoutingResult {
    let mut headers_body = request.split("\r\n\r\n");
    let _headers = headers_body.next().expect("No headers were sent with request");
    let body = headers_body.next();
    match body {
        Some(body) =>
            RoutingResult::User(Action::Auth, ACCESS_CONTROL_HEADERS.to_string(), body.to_string()),
        None => {
            error!("Body of email request appears empty");
            RoutingResult::Err("400".to_string(), "Bad Request".to_string(), request.to_string())
        }
    }
}