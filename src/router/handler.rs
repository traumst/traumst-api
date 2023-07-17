use crate::email;

pub fn send_email(request: &str) -> Result<String, String> {
    let mut headers_body = request.split("\r\n\r\n");
    let _headers = headers_body.next().unwrap();
    let body = headers_body.next().unwrap();

    match email::parse_request(body) {
        Ok(json) => { email::send_email(json) }
        Err(msg) => { Err(msg) }
    }
}