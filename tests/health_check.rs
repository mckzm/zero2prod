// 4 properties must hold:
// 1) the health check is exposed at `/health_check'
// 2) the health check requires a GET
// 3) the health check returns a `200 OK` HTTP status code
// 4) the health check's response has no body

use std::net::TcpListener;
use zero2prod::startup::run;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to socket");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind to given socket");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn test_health_check_works() {
    let socket_spec = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", socket_spec))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.content_length(), Some(0));
}
