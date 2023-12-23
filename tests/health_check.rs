use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> TestApp {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Could not bind to port");

    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);

    let configuration = configuration::get_configuration()
        .expect("Failed to get configuration");

    let connection_pool =
        PgPool::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to database");

    let server = zero2prod::startup::run(listener, connection_pool.clone())
        .expect("Failed to start server");

    tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}
