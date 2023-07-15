mod router;
mod response;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let http_request = std::str::from_utf8(&buffer).unwrap();
    let routing_result = router::handle_request(http_request);
    let http_response = response::generate_for(routing_result);

    stream.write(http_response.as_bytes()).unwrap();
    stream.flush().unwrap();
}