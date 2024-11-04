// 4 properties must hold:
// 1) the health check is exposed at `/health_check'
// 2) the health check requires a GET
// 3) the health check returns a `200 OK` HTTP status code
// 4) the health check's response has no body

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind to given socket");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn test_health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
