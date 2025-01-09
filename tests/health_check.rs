use zero2prod::create_server;

#[tokio::test]
async fn when_health_check_called_then_returns_200() {
    spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = create_server().expect("Failed to create server");
    tokio::spawn(server);
}
