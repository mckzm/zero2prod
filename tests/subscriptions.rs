// 3 properties must hold:
// 1) A valid request is a POST to `/subscriptions` that includes both name and
// email address submitted in `application/x-www-form-urlencoded` format
// 2) A valid request should return a `200 OK`
// 3) If either the name or the email address is missing, return
// `400 BAD REQUEST`

mod common;

#[tokio::test]
async fn test_subscriptions_returns_200_for_a_valid_request() {
    let socket_spec = common::spawn_app();
    let client = reqwest::Client::new();

    let post_body = "name=Zephyra%20Kaze&email=zephyra_kaze%40.example.com";
    let response = client
        .post(&format!("{}/subscriptions", socket_spec))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(post_body)
        .send()
        .await
        .expect("Failed to execute POST request");

    assert_eq!(response.status().as_u16(), 200);
}
