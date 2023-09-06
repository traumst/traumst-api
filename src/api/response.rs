use std::fmt;

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

pub fn generate_for(response: Response) -> String {
    format!(
        "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
        response.status_code,
        response.status_message,
        response.headers,
        response.body)
}

pub fn generate_error(err: &str, body: String) -> Response {
    match err {
        "Bad Request" => Response {
            status_code: "400".to_string(),
            status_message: "Bad Request".to_string(),
            headers: "".to_string(),
            body
        },
        "Not Found" => Response {
            status_code: "404".to_string(),
            status_message: "Not Found".to_string(),
            headers: "".to_string(),
            body
        },
        "Gateway Timeout" => Response {
            status_code: "504".to_string(),
            status_message: "Gateway Timeout".to_string(),
            headers: "".to_string(),
            body
        },
        _ => Response {
            status_code: "500".to_string(),
            status_message: "Internal Server Error".to_string(),
            headers: "".to_string(),
            body,
        },
    }
}