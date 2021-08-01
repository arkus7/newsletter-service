use newsletter_service::configuration::get_configuration;
use newsletter_service::startup::run;
use newsletter_service::telemetry::{get_logs_subscriber, init_logs_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let logs_subscriber =
        get_logs_subscriber("newsletter_service".into(), "info".into(), std::io::stdout);
    init_logs_subscriber(logs_subscriber);

    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to the database");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to port");
    run(listener, connection_pool)?.await
}
