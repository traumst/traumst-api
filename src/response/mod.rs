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