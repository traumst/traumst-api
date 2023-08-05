use log::{error, trace};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::{response, router};
use crate::response::Response;

pub async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    match stream.read(&mut buffer).await {
        Ok(bytes_read) => match std::str::from_utf8(&buffer[..bytes_read]) {
            Ok(http_request) => write_output(stream, http_request).await,
            Err(e) => error!("Failed to read input into string: {e:?}"),
        }
        Err(e) => error!("Failed to read input stream {e:?}"),
    }
}

async fn write_output(mut stream: TcpStream, http_request: &str) {
    let result = process(http_request);
    match stream.write_all(result.as_bytes()).await {
        Ok(_) => trace!("Response output written"),
        Err(e) => error!("Failed to write response output {e:?}"),
    }
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