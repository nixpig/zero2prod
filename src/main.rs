use std::net::TcpListener;

use zero2prod::{configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = configuration::get_configuration()
        .expect("Failed to read configuration file");

    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", config.application_port))
            .expect("Could not bind to port");

    let connection_string = config.database.connection_string();

    let pool = sqlx::PgPool::connect(&connection_string)
        .await
        .expect("failed to connect to database");

    run(listener, pool)?.await
}
