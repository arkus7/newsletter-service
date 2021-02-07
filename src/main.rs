use newsletter_service::configuration::get_configuration;
use newsletter_service::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect(&format!(
        "Failed to bind to port {}",
        config.application_port
    ));
    run(listener)?.await
}
