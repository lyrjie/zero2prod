use zero2prod::create_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    create_server()?.await
}
