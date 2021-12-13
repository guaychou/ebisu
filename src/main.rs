use ebisu::application;
use ebisu::cli;
use ebisu::configuration;
use ebisu::domain::telegram::Telegram;
use ebisu::log;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let cli = cli::Options::new();
    log::log_init();
    let config = configuration::read_config(cli.get_config_path());
    let telegram = Telegram::new(config.telegram);
    let addr: SocketAddr = SocketAddr::from((
        "0.0.0.0".parse::<std::net::Ipv4Addr>().unwrap(),
        *config.server.get_port(),
    ));
    let server = axum::Server::bind(&addr)
        .serve(
            application::build(config.server, telegram)
                .into_make_service_with_connect_info::<SocketAddr, _>(),
        )
        .with_graceful_shutdown(application::shutdown_signal());
    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }
}
