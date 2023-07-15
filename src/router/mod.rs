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
        "/hello" => Ok("Hello, world!".to_string()),
        _ => Err("Not found".to_string()),
    }
}