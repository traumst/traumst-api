mod router;
mod response;
mod request;
mod config;
mod email;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let host = "0.0.0.0";
    let port = config::listen_on_port();
    let listen_on = format!("{}:{}", host, port);
    let listener = TcpListener::bind(listen_on.clone())
        .expect("Listener fail to bind");
    println!("server is listening on {listen_on:?}");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { handle_client(stream); }
            Err(err) => { println!("Failed while consuming incoming stream: {err:?}") }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let http_request = std::str::from_utf8(&buffer[..bytes_read])
        .expect("Failed to read input into string");

    let http_response = request::process(http_request);

    stream.write(http_response.as_bytes()).unwrap();
    stream.flush().unwrap();
}