use crate::response::Response;

mod handler;

pub fn handle_request(request: &str) -> Result<Response, String> {
    let parts: Vec<&str> = request.split_whitespace().collect();
    let method = parts[0];
    let path = parts[1];

    match method {
        "OPTIONS" => handle_options_request(path),
        "GET" => handle_get_request(path),
        "POST" => handle_post_request(path, request),
        _ => Err("Not found".to_string()),
    }
}

fn handle_options_request(path: &str) -> Result<Response, String> {
    match path {
        "/email" => handler::send_options(),
        _ => Err("Not found".to_string()),
    }
}

fn handle_get_request(path: &str) -> Result<Response, String> {
    match path {
        "/ping" => handler::send_pong(),
        _ => Err("Not found".to_string()),
    }
}

fn handle_post_request(path: &str, request: &str) -> Result<Response, String> {
    match path {
        "/email" => handler::send_email(request),
        _ => Err("Not found".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_get_request() {
        let result = handle_get_request("/ping").unwrap();
        assert_eq!(result.body, "pong");
    }

    #[test]
    fn test_bad_handle_get_request() {
        assert_eq!(handle_get_request("/gibberish").is_err(), true);
    }
}