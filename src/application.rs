use crate::configuration::ServerConfig;
use crate::domain::telegram::Telegram;
use crate::error::handle_error;
use crate::handler::{alert::alert, alert_with_message::alert_with_message};
use axum::routing::BoxRoute;
use axum::AddExtensionLayer;
use axum::{handler::post, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn build(config: ServerConfig, telegram: Telegram) -> Router<BoxRoute> {
    let middleware_stack = ServiceBuilder::new()
        .load_shed()
        .buffer(*config.get_buffer())
        .concurrency_limit(*config.get_concurrency_limit())
        .timeout(*config.get_timeout())
        .rate_limit(*config.get_rate_limit(), *config.get_limiter_timeout())
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(telegram))
        .into_inner();

    Router::new()
        .route("/api/v1/alert", post(alert))
        .route("/api/v1/alertwithmessage", post(alert_with_message))
        .layer(middleware_stack)
        .handle_error(handle_error)
        .boxed()
}

pub async fn shutdown_signal() {
    use std::io;
    use tokio::signal::unix::SignalKind;
    async fn terminate() -> io::Result<()> {
        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;
        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    tracing::info!("signal received, starting graceful shutdown")
}