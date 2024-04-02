use std::fmt;

const ACCESS_CONTROL_HEADERS: &str = r#"Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: Content-Type, Content-Length"#;

#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: String,
    pub status_message: String,
    pub headers: String,
    pub body: String,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub fn serialize(response: Response) -> String {
    format!(
        "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
        response.status_code,
        response.status_message,
        response.headers,
        response.body)
}

pub fn ok200(body: String) -> Response {
    Response {
        status_code: "200".to_string(),
        status_message: "OK".to_string(),
        headers: ACCESS_CONTROL_HEADERS.to_string(),
        body,
    }
}

pub fn ok204() -> Response {
    Response {
        status_code: "204".to_string(),
        status_message: "No Content".to_string(),
        headers: ACCESS_CONTROL_HEADERS.to_string(),
        body: String::new(),
    }
}

pub fn err400(message: String) -> Response {
    Response {
        status_code: "400".to_string(),
        status_message: "Bad Request".to_string(),
        headers: ACCESS_CONTROL_HEADERS.to_string(),
        body: message,
    }
}

pub fn _err404(message: String) -> Response {
    Response {
        status_code: "404".to_string(),
        status_message: "Bad Request".to_string(),
        headers: ACCESS_CONTROL_HEADERS.to_string(),
        body: message,
    }
}

pub fn err500(message: String) -> Response {
    Response {
        status_code: "500".to_string(),
        status_message: "Internal Server Error".to_string(),
        headers: ACCESS_CONTROL_HEADERS.to_string(),
        body: message,
    }
}

pub fn err501() -> Response {
    Response {
        status_code: "501".to_string(),
        status_message: "Not Implemented Yet".to_string(),
        headers: ACCESS_CONTROL_HEADERS.to_string(),
        body: String::new(),
    }
}

pub fn _err504(message: String) -> Response {
    Response {
        status_code: "504".to_string(),
        status_message: "Gateway Timeout".to_string(),
        headers: ACCESS_CONTROL_HEADERS.to_string(),
        body: message,
    }
}