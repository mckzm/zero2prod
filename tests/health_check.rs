// 4 properties must hold:
// 1) the health check is exposed at `/health_check'
// 2) the health check requires a GET
// 3) the health check returns a `200 OK` HTTP status code
// 4) the health check's response has no body

mod common;

#[tokio::test]
async fn test_health_check_works() {
    let socket_spec = common::spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", socket_spec))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.content_length(), Some(0));
}
