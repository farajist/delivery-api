#[tokio::test]
async fn health_check_works() {
    // arrange
    spawn_app();

    let client = reqwest::Client::new();

    // act
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = delivery_api::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}
