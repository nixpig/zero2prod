use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Could not bind to port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Could not start server");

    tokio::spawn(server);

    let address = format!("http://127.0.0.1:{}", &port);

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Could not bind to port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Could not start server");

    tokio::spawn(server);

    let address = format!("http://127.0.0.1:{}", &port);

    let client = reqwest::Client::new();

    let body = "name=le&20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_form_data() {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Could not bind to port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Could not start server");

    tokio::spawn(server);

    let address = format!("http://127.0.0.1:{}", port);

    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email field"),
        ("email=ursula_le_guin%40gmail.com", "missing the name field"),
        ("", "missing both the name and email fields"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

// fn spawn_app() -> String {
//     // :0 binds to a random available port at OS level
//     let listener = TcpListener::bind("127.0.0.1:0")
//         .expect("Could not bind to random port");
//
//     let port = listener.local_addr().unwrap().port();
//
//     let server = zero2prod::run(listener).expect("Failed to bind to listener");
//
//     tokio::spawn(server);
//
//     format!("http://127.0.0.1:{}", port)
// }
