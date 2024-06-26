use log::{
    debug,
    trace,
    info,
    error,
};
use tokio::{
    net::TcpListener,
    net::TcpStream,
    io::AsyncReadExt,
    io::AsyncWriteExt,
};
use crate::{
    config,
    infra,
    api::response,
    api
};

pub struct Server {
    //db: Arc<db::pool::Bridge>,
}

impl Server {
    pub async fn new() -> Self {
        infra::logger::init();
        Self{}
    }

    pub async fn init(self) {
        let host = "0.0.0.0";
        let port = config::listen_on_port();
        let listen_on = format!("{}:{}", host, port);
        let listener = TcpListener::bind(listen_on.clone())
            .await
            .expect("Listener fail to bind");
        info!("Server is listening on {listen_on:?}");

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        //let chat_app = self.chat.clone();
                        tokio::spawn(async move {
                            debug!("  processing incoming request");
                            handle_input(stream).await
                        });
                    }
                    Err(err) => error!("failed to read from socket; err: {err:?}"),
                }
            }
        });
    }
}

async fn handle_input (
    mut stream: TcpStream
) {
    let mut buffer = [0; 2048];
    match stream.read(&mut buffer).await {
        Ok(bytes_read) => match std::str::from_utf8(&buffer[..bytes_read]) {
            Ok(http_request) => handle_request(stream, http_request).await,
            Err(e) => error!("Failed to read input into string: {e:?}"),
        }
        Err(e) => error!("Failed to read input stream {e:?}"),
    }
}

async fn handle_request(
    mut stream: TcpStream,
    http_request: &str,
) {
    let result = api::handle(http_request).await;
    let response = serialize(result);
    match stream.write_all(response.as_bytes()).await {
        Ok(_) => trace!("Response output written"),
        Err(e) => error!("Failed to write response output {e:?}"),
    }
}

fn serialize(
    routing_result: Result<response::Response, response::Response>
) -> String {
    let result = match routing_result {
        Ok(routing) => response::Response {
            status_code: routing.status_code,
            status_message: routing.status_message,
            headers: routing.headers,
            body: routing.body
        },
        Err(error) => error
    };
    response::serialize(result)
}