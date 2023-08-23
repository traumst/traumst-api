use std::sync::Arc;
use crate::db::pool::Bridge;
use super::response::Response;

mod route;
mod process;

pub const ACCESS_CONTROL_HEADERS: &str = r#"Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: Content-Type, Content-Length"#;

pub enum RoutingResult {
    User(String, String, u32),
    Email(String, String),
    Pong(String, String),
    Options(String),
    Err(String, String, String),
}

pub async fn route(http_request: &str, shared_pool: Arc<Bridge>) -> Result<Response, Response> {
    let res = route::direct(http_request).await;
    match res {
        RoutingResult::User(head, body, user_id) => {
            if user_id == 0 {
                process::user::create(head.as_str(), body.as_str(), shared_pool).await
            } else {
                process::user::get(head.as_str(), user_id, shared_pool).await
            }
        }
        RoutingResult::Email(_, body) => {
            process::email::send(body.as_str())
        }
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