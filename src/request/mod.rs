use crate::{response, router};
use crate::response::Response;

pub fn process(http_request: &str) -> String {
    let routing_result = router::handle_request(http_request);
    let result = translate(routing_result);
    println!("sending response {result}");

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