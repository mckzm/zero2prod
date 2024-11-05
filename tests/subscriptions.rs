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

#[tokio::test]
async fn test_subscriptions_returns_400_for_invalid_requests() {
    let socket_spec = common::spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Zephyra%20Kaze", "Email address is missing"),
        ("email=zephyra_kaze%40.example.com", "Name is missing"),
        ("", "Name and email address are both missing"),
    ];

    for (invalid_body, error_msg) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", socket_spec))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute POST request");

        assert_eq!(
            response.status().as_u16(),
            400,
            "The handler did not return 400 when the payload had the following flaw: {}",
            error_msg
        );
    }
}
