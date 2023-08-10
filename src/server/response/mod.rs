pub struct Response {
    pub status_code: &'static str,
    pub status_message: &'static str,
    pub headers: String,
    pub body: String,
}

pub fn generate_for(response: Response) -> String {
    format!(
        "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
        response.status_code,
        response.status_message,
        response.headers,
        response.body)
}

pub fn error(err: String) -> Response {
    match err.as_str() {
        "Bad request" => Response {
            status_code: "400",
            status_message: "Bad Request",
            headers: "".to_string(),
            body: err
        },
        "Not found" => Response {
            status_code: "404",
            status_message: "Not Found",
            headers: "".to_string(),
            body: err
        },
        "Gateway Timeout" => Response {
            status_code: "504",
            status_message: "Gateway Timeout",
            headers: "".to_string(),
            body: err
        },
        _ => Response {
            status_code: "500",
            status_message: "Internal Server Error",
            headers: "".to_string(),
            body: err,
        },
    }
}