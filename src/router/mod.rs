pub fn handle_request(request: &str) -> Option<Result<String, String>> {
    let parts: Vec<&str> = request.split_whitespace().collect();
    let method = parts[0];
    let path = parts[1];

    match method {
        "GET" => Some(handle_get_request(path)),
        _ => Some(Err("Not implemented".to_string())),
    }
}

fn handle_get_request(path: &str) -> Result<String, String> {
    match path {
        "/ping" => Ok("pong".to_string()),
        _ => Err("Not found".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_get_request() {
        let result = handle_get_request("/ping");
        assert_eq!(result.unwrap(), "pong");
    }

    #[test]
    fn test_bad_handle_get_request() {
        assert_eq!(handle_get_request("/bad").is_err(), true);
    }
}