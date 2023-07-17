pub struct Response {
    pub status_code: &'static str,
    pub status_message: &'static str,
    pub body: String,
}

pub fn generate_for(routing_result: Result<String, String>) -> String {
    let response = generate_response(routing_result);
    format!(
        "HTTP/1.1 {} {}\r\n\r\n{}",
        response.status_code,
        response.status_message,
        response.body)
}

fn generate_response(routing_result: Result<String, String>) -> Response {
    match routing_result {
        Ok(body) => Response {
            status_code: "200",
            status_message: "OK",
            body
        },
        Err(error) => generate_error(error),
    }
}

fn generate_error(error: String) -> Response {
    match error.as_str() {
        "Bad request" => Response {
            status_code: "400",
            status_message: "Bad Request",
            body: error
        },
        "Not found" => Response {
            status_code: "404",
            status_message: "Not Found",
            body: error
        },
        "Gateway Timeout" => Response {
            status_code: "504",
            status_message: "Gateway Timeout",
            body: error
        },
        _ => Response {
            status_code: "500",
            status_message: "Internal Server Error",
            body: error,
        },
    }
}