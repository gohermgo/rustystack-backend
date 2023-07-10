use std::net::TcpListener;

#[tokio::test]
async fn health_check_successful() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Request failed");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn greet_noone_successful() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/greet", &address))
        .send()
        .await
        .expect("request failed");
    assert!(response.status().is_success());
    println!("response {}", response.text().await.ok().unwrap());
}

// #[tokio::test]
// async fn greet_someone_successful() {
//     // Arrange
//     spawn_app().await.expect("Failed to spawn app");
//     let client = reqwest::Client::new();
//     // Act
//     let response = client
//         .get(format!("{}greet/steve", site_uri()))
//         .send()
//         .await
//         .expect("request failed");
//     assert!(response.status().is_success());
//     println!("response {}", response.text().await.ok().unwrap());
//     // assert_eq!(Some(5), response.content_length());
// }

fn spawn_app() -> String {
    use rustystack_backend::{run, PUBLIC_IP};
    let listener =
        TcpListener::bind(format!("{}:0", PUBLIC_IP)).expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://{}:{}", PUBLIC_IP, port)
}
