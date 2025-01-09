use std::net::TcpListener;
use zero2prod::create_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("http://127.0.0.1:8000").expect("Failed to bind listener");
    create_server(listener)?.await
}
