use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Could not bind to port");

    let port = listener.local_addr().unwrap().port();

    let server =
        zero2prod::startup::run(listener).expect("Could not start server");

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

// fn spawn_app() -> String {
//     // :0 binds to a random available port at OS level
//     let listener = TcpListener::bind("127.0.0.1:0")
//         .expect("Could not bind to random port");
//
//     let port = listener.local_addr().unwrap().port();
//
//     let server = zero2prod::startup::run(listener).expect("Failed to bind to listener");
//
//     tokio::spawn(server);
//
//     format!("http://127.0.0.1:{}", port)
// }
