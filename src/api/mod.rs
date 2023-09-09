use std::sync::Arc;
use super::db::pool::Bridge;
use response::Response;

pub mod response;
pub mod model;
mod router;
mod handler;

pub const ACCESS_CONTROL_HEADERS: &str = r#"Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: Content-Type, Content-Length"#;

pub enum RoutingResult {
    Chat(handler::chat::ChatAction, String, String),
    User(handler::user::UserAction, String, String),
    Email(String, String),
    Pong(String, String),
    Options(String),
    Err(String, String, String),
}

pub async fn route(http_request: &str, shared_pool: Arc<Bridge>) -> Result<Response, Response> {
    let res = router::direct(http_request).await;
    match res {
        RoutingResult::Chat(action, head, body) =>
            handler::chat::process(action, head.as_str(), body.as_str(), shared_pool).await,
        RoutingResult::User(action, head, body) =>
            handler::user::process(action, head.as_str(), body.as_str(), shared_pool).await,
        RoutingResult::Email(_, body) =>
            handler::email::send(body.as_str()),
        RoutingResult::Pong(head, body) => Ok(Response {
            status_code: "200".to_string(),
            status_message: "Ok".to_string(),
            headers: head.to_string(),
            body: body.to_string(),
        }),
        RoutingResult::Options(head) => Ok(Response {
            status_code: "204".to_string(),
            status_message: "No Content".to_string(),
            headers: head.to_string(),
            body: "".to_string(),
        }),
        RoutingResult::Err(code, status, body) => Err(Response {
            status_code: code,
            status_message: status,
            headers: "".to_string(),
            body: body.to_string(),
        })
    }
}