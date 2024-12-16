use std::net::TcpListener;

use cloudcafe::{configuration::get_configuration, startup::run, telemetry::{get_subscriber, init_subscriber}};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("cloudcafe".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy_with(configuration.database.get_connect_options());

    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind to address");

    run(listener, connection_pool)?.await
}
