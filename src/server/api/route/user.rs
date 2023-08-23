use std::str::FromStr;
use log::error;
use super::RoutingResult;
use super::ACCESS_CONTROL_HEADERS;

pub async fn get(path: &str) -> RoutingResult {
    let arg_id = path.split('/').find(|&p| p.parse::<u32>().is_ok());
    if arg_id.is_none() {
        error!("Missing mandatory argument");
        return RoutingResult::Err("400".to_string(), "Bad Request".to_string(), path.to_string());
    }

    let user_id = u32::from_str(arg_id.unwrap_or("0"));
    if user_id.is_err() {
        error!("Invalid argument value");
        return RoutingResult::Err("400".to_string(), "Bad Request".to_string(), path.to_string());
    }

    return RoutingResult::User(ACCESS_CONTROL_HEADERS.to_string(), "".to_string(), user_id.unwrap());
}

pub async fn create(request: &str) -> RoutingResult {
    let mut headers_body = request.split("\r\n\r\n");
    let _headers = headers_body.next().expect("No headers were sent with request");
    match headers_body.next() {
        Some(body) => {
            RoutingResult::User(ACCESS_CONTROL_HEADERS.to_string(), body.to_string(), 0)
        }
        None => {
            error!("Body of email request appears empty");
            return RoutingResult::Err("400".to_string(), "Bad Request".to_string(), request.to_string());
        }
    }
}