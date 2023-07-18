mod router;
mod response;
mod request;
mod config;
mod email;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let host = "127.0.0.1";
    let port = config::listen_on_port();
    let listen_on = format!("{}:{}", host, port);
    let listener = TcpListener::bind(listen_on.clone()).unwrap();
    println!("server is listening on {listen_on:?}");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();

    let http_response = request::process(buffer, bytes_read);

    stream.write(http_response.as_bytes()).unwrap();
    stream.flush().unwrap();
}