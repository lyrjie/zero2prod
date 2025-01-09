use std::net::TcpListener;
use zero2prod::create_server;

#[tokio::test]
async fn when_health_check_called_then_returns_200() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind listener");
    let port = listener.local_addr().unwrap().port();
    let server = create_server(listener).expect("Failed to create server");
    tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
