use log::error;
use super::RoutingResult;

pub fn send(request: &str) -> RoutingResult {
    let mut headers_body = request.split("\r\n\r\n");
    let _headers = headers_body.next().expect("No headers were sent with request");
    match headers_body.next() {
        Some(body) => {
            RoutingResult::Email(body.to_string())
        }
        None => {
            error!("Body of email request appears empty");
            RoutingResult::Err(
                "400".to_string(),
                "Bad Request".to_string(),
                request.to_string())
        }
    }
}