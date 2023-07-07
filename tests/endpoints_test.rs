// use crate::*;

// #[tokio::test]
// async fn health_check_succeeds() {
//     let response = backend::health_check().await;
//     assert!(response.status().is_success())
// }
// me putting integration tests

use rustystack_backend::{run, site_uri};

#[tokio::test]
async fn health_check_successful() {
    // Arrange
    spawn_app().await.expect("Failed to spawn app");
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(format!("{}/health_check", site_uri))
        .send()
        .await
        .expect("Request failed");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
async fn spawn_app() -> std::io::Result<()> {
    todo!()
}
