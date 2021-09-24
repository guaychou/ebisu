use axum::AddExtensionLayer;
use axum::{handler::post, Router};
use ebisu::cli;
use ebisu::configuration;
use ebisu::domain::telegram::Telegram;
use ebisu::error::handle_error;
use ebisu::handler::{alert::alert, alert_with_message::alert_with_message};
use ebisu::log;
// use ebisu::util::service_util_init;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    trace::TraceLayer,
};

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
                        
                .load_shed()
                .buffer(*config.server.get_buffer())
                .concurrency_limit(*config.server.get_concurrency_limit())
                .timeout(*config.server.get_timeout())
                .rate_limit(
                    *config.server.get_rate_limit(),
                    *config.server.get_limiter_timeout(),
                )
                .into_inner(),
        )
        .handle_error(handle_error);

    let addr = SocketAddr::from(([0, 0, 0, 0], *config.server.get_port()));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
