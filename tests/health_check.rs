//! tests/health_check.rs

#[tokio::test]
async fn health_check_works() {
    //Spawn
    spawn_app().await.expect("Failed to spawn app.");

    // Request
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/healt_check")
        .send()
        .await
        .expect("Failed to execute health check request...");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    todo!()
}
