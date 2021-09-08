use axum::AddExtensionLayer;
use axum::{handler::post, Router};
use ebisu::cli;
use ebisu::configuration;
use ebisu::domain::telegram::Telegram;
use ebisu::handler::{
    alert::alert,
    alert_with_message::alert_with_message};
use ebisu::log;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let cli = cli::Options::new();
    log::log_init();
    let config = configuration::read_config(cli.get_config_path());
    let telegram = Telegram::new(config.telegram);
    let app = Router::new()
        .route("/api/v1/alert", post(alert))
        .route("/api/v1/alertwithmessage", post(alert_with_message))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(telegram))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );
    let addr = SocketAddr::from(([0, 0, 0, 0], *config.server.get_port()));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
