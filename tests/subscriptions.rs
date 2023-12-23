use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::{self, get_configuration};

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Could not bind to port");

    let port = listener.local_addr().unwrap().port();

    let configuration =
        get_configuration().expect("failed to get configuration");

    let connection_string = configuration.database.connection_string();

    let connection = PgPool::connect(&connection_string)
        .await
        .expect("failed to connect to database");

    let address = format!("http://127.0.0.1:{}", &port);

    let server = zero2prod::startup::run(listener, connection)
        .expect("Could not start server");

    tokio::spawn(server);

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

    // let saved = sqlx::query!("select name, email from subscriptions")
    //     .fetch_one(connection)
    //     .await
    //     .expect("Failed to fetch saved subscription");

    // assert_eq!(saved.name, "le guin");
    // assert_eq!(saved.email, "ursula_le_guin@gmail.com");
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_form_data() {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Could not bind to port");

    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);

    let configuration = configuration::get_configuration()
        .expect("failed to get configutation");
    let connection_string = configuration.database.connection_string();

    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    let server = zero2prod::startup::run(listener, pool)
        .expect("Could not start server");

    tokio::spawn(server);

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
