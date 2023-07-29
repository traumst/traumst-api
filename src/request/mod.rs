use std::io::{Read, Write};
use std::net::TcpStream;
use crate::{response, router};
use crate::response::Response;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let http_request = std::str::from_utf8(&buffer[..bytes_read])
        .expect("Failed to read input into string");

    let http_response = process(http_request);

    stream.write(http_response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn process(http_request: &str) -> String {
    let routing_result = router::handle_request(http_request);
    let result = translate(routing_result);

    result
}

fn translate(routing_result: Result<Response, String>) -> String {
    let result = match routing_result {
        Ok(routing) => Response {
            status_code: routing.status_code,
            status_message: routing.status_message,
            headers: routing.headers,
            body: routing.body
        },
        Err(error) => generate_error(error),
    };

    response::generate_for(result)
}

fn generate_error(error: String) -> Response {
    match error.as_str() {
        "Bad request" => Response {
            status_code: "400",
            status_message: "Bad Request",
            headers: "".to_string(),
            body: error
        },
        "Not found" => Response {
            status_code: "404",
            status_message: "Not Found",
            headers: "".to_string(),
            body: error
        },
        "Gateway Timeout" => Response {
            status_code: "504",
            status_message: "Gateway Timeout",
            headers: "".to_string(),
            body: error
        },
        _ => Response {
            status_code: "500",
            status_message: "Internal Server Error",
            headers: "".to_string(),
            body: error,
        },
    }
}