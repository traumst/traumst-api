pub struct Response {
    pub status_code: &'static str,
    pub status_message: &'static str,
    pub body: String,
}

pub fn generate_for(routing_result: Option<Result<String, String>>) -> String {
    let response = generate_response(routing_result);
    format!(
        "HTTP/1.1 {} {}\r\n\r\n{}",
        response.status_code,
        response.status_message,
        response.body)
}

fn generate_response(routing_result: Option<Result<String, String>>) -> Response {
    match routing_result {
        Some(Ok(body)) => Response {
            status_code: "200",
            status_message: "OK",
            body
        },
        Some(Err(error)) => generate_error(error),
        None => Response {
            status_code: "400",
            status_message: "Bad Request",
            body: String::new(),
        },
    }
}

fn generate_error(error: String) -> Response {
    match error.as_str() {
        "Not found" => Response {
            status_code: "404",
            status_message: "Not Found",
            body: error
        },
        _ => Response {
            status_code: "501",
            status_message: "Not Implemented",
            body: error,
        },
    }
}