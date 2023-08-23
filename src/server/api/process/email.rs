use crate::infra::email;
use crate::server::response;

pub fn send(body: &str) -> Result<response::Response, response::Response> {
    match email::parse_request(body) {
        Ok(json) => { handle_success(json) }
        Err(msg) => {
            Err(response::Response {
                status_code: "400".to_string(),
                status_message: "Bad Request".to_string(),
                headers: "".to_string(),
                body: msg,
            })
        }
    }
}

fn handle_success(json: email::EmailRequest) -> Result<response::Response, response::Response> {
    match email::send_email(json) {
        Ok(body) => Ok(response::Response {
            status_code: "200".to_string(),
            status_message: "OK".to_string(),
            headers: "".to_string(),
            body,
        }),
        Err(body) => Err(response::Response {
            status_code: "500".to_string(),
            status_message: "Internal Server Error".to_string(),
            headers: "".to_string(),
            body,
        })
    }
}