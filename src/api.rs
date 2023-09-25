use std::sync::Arc;
use super::db::pool::Bridge;

pub mod response;
pub mod model;
mod router;
mod handler;

pub enum RoutingResult {
    Chat(handler::chat::ChatAction, String),
    User(handler::user::UserAction, String),
    Email(String),
    Pong(String),
    Options,
    Err(String, String, String),
}

pub async fn handle(http_request: &str, shared_pool: Arc<Bridge>) -> Result<response::Response, response::Response> {
    let res = router::route(http_request).await;
    match res {
        RoutingResult::Chat(action, body) =>
            handler::chat::process(
                action,
                body.as_str(),
                shared_pool
            ).await,
        RoutingResult::User(action, body) =>
            handler::user::process(
                action,
                body.as_str(),
                shared_pool
            ).await,
        RoutingResult::Email(body) =>
            handler::email::send(body.as_str()),
        // TODO provide head
        RoutingResult::Pong(body) => Ok(response::ok200(body.to_string())),
        // TODO provide head
        RoutingResult::Options => Ok(response::ok204()),
        // TODO replace this
        RoutingResult::Err(code, status, body) => Err(response::Response {
            status_code: code,
            status_message: status,
            headers: "".to_string(),
            body: body.to_string(),
        })
    }
}