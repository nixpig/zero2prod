use std::net::TcpListener;

use zero2prod::{configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = configuration::get_configuration()
        .expect("Failed to read configuration file");

    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", config.application_port))
            .expect("Could not bind to port");

    run(listener)?.await
}
