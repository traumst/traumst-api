use crate::email;
use crate::response::Response;

pub fn send_pong() -> Result<Response, String> {
    println!("sending pong...");
    Ok(Response {
        status_code: "204",
        status_message: "204",
        headers: "".to_string(),
        body: "pong".to_string(),
    })
}

pub fn send_options() -> Result<Response, String> {
    println!("sending options...");
    Ok(Response {
        status_code: "204",
        status_message: "No Content",
        headers: r#"Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: Content-Type, Content-Length"#.to_string(),
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
                    email::send_email(json).expect("Failed to send an email");
                    Ok(Response {
                        status_code: "204",
                        status_message: "No Content",
                        headers: "".to_string(),
                        body: "Done".to_string(),
                    })
                }
                Err(msg) => { Err(msg) }
            }
        }
    }
}