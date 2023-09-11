use log::error;
use crate::api::handler::chat::ChatAction;
use crate::api::RoutingResult;

pub async fn create(request: &str) -> RoutingResult {
    let mut headers_body = request.split("\r\n\r\n");
    let _headers = match headers_body.next() {
        Some(headers) => headers,
        None => {
            return RoutingResult::Err(
                "400".to_string(),
                "Bad Request".to_string(),
                "No headers were sent with request".to_string());
        }
    };

    match headers_body.next() {
        Some(body) => RoutingResult::Chat(
            ChatAction::Create,
            body.to_string()),
        None => {
            error!("Body of email request appears empty");
            return RoutingResult::Err(
                "400".to_string(),
                "Bad Request".to_string(),
                request.to_string());
        }
    }
}

pub async fn join(_request: &str) -> RoutingResult {
    todo!()
}

pub async fn send(_request: &str) -> RoutingResult {
    todo!()
}