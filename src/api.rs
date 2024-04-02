pub mod response;
mod router;
mod handler;

pub enum RoutingResult {
    _Chat(handler::chat::ChatAction, String),
    Email(String),
    Pong(String),
    Options,
    Err(String, String, String),
}

pub async fn handle(http_request: &str) -> Result<response::Response, response::Response> {
    let res = router::route(http_request).await;
    match res {
        RoutingResult::_Chat(_action, _body) => Ok(
            response::ok200("to be continued...".to_string())),
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