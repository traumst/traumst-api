use crate::infra::email;
use crate::api::response;

pub fn send(body: &str) -> Result<response::Response, response::Response> {
    match email::parse_request(body) {
        Ok(json) => handle_success(json),
        Err(msg) => Err(response::err400(msg))
    }
}

fn handle_success(json: email::EmailRequest) -> Result<response::Response, response::Response> {
    match email::send_email(json) {
        Ok(body) => Ok(response::ok200(body)),
        Err(body) => Err(response::err500(body))
    }
}