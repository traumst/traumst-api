use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::pool::Bridge;
use response::Response;

pub mod response;
mod router;
mod handler;

pub const ACCESS_CONTROL_HEADERS: &str = r#"Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: Content-Type, Content-Length"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Create,
    Auth,
}

pub enum RoutingResult {
    User(Action, String, String),
    Email(String, String),
    Pong(String, String),
    Options(String),
    Err(String, String, String),
}

pub async fn route(http_request: &str, shared_pool: Arc<Bridge>) -> Result<Response, Response> {
    let res = router::direct(http_request).await;
    match res {
        RoutingResult::User(crud, head, body) => match crud {
            Action::Create => handler::user::create(head.as_str(), body.as_str(), shared_pool).await,
            Action::Auth => handler::user::auth(head.as_str(), body.as_str(), shared_pool).await,
        }
        RoutingResult::Email(_, body) => handler::email::send(body.as_str()),
        RoutingResult::Options(head) => Ok(Response {
            status_code: "204".to_string(),
            status_message: "No Content".to_string(),
            headers: head.to_string(),
            body: "".to_string(),
        }),
        RoutingResult::Pong(head, body) => Ok(Response {
            status_code: "200".to_string(),
            status_message: "Ok".to_string(),
            headers: head.to_string(),
            body: body.to_string(),
        }),
        RoutingResult::Err(code, status, body) => Err(Response {
            status_code: code,
            status_message: status,
            headers: "".to_string(),
            body: body.to_string(),
        })
    }
}