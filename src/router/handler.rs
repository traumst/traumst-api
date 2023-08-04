use crate::email;
use crate::response::Response;

pub fn send_pong() -> Result<Response, String> {

    Ok(Response {
        status_code: "200",
        status_message: "200",
        headers: "".to_string(),
        body: "pong".to_string(),
    })
}

const ACCESS_CONTROL_HEADERS: &str = r#"Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: Content-Type, Content-Length"#;

pub fn send_options() -> Result<Response, String> {
    Ok(Response {
        status_code: "204",
        status_message: "No Content",
        headers: ACCESS_CONTROL_HEADERS.to_string(),
        body: "".to_string(),
    })
}

pub fn send_email(request: &str) -> Result<Response, String> {
    let mut headers_body = request.split("\r\n\r\n");
    let _headers = headers_body.next().expect("No headers were sent with request");
    match headers_body.next() {
        None => { Err("Body of request appears empty".to_string())}
        Some(body) => {
            match email::parse_request(body) {
                Ok(json) => {
                    match email::send_email(json) {
                        Ok(res) => Ok(Response {
                            status_code: "200",
                            status_message: "OK",
                            headers: ACCESS_CONTROL_HEADERS.to_string(),
                            body: res,
                        }),
                        Err(err) => Err(err)
                    }
                }
                Err(msg) => { Err(msg) }
            }
        }
    }
}