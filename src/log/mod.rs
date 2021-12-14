use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, Registry, fmt::{format, time::LocalTime}};
use {env, figlet_rs::FIGfont, log::info};

pub fn log_init() {
    let time_format = LocalTime::rfc_3339();
    let tracing_format = format().with_timer(time_format);
    let fmt_layer = tracing_subscriber::fmt::Layer::default().event_format(tracing_format);
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "ebisu=info")
    }
    let jaeger_address = std::env::var("JAEGER_ADDRESS").unwrap_or(String::from("localhost:6831"));
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .with_agent_endpoint(jaeger_address)
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let collector = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(fmt_layer)
        .with(opentelemetry);
    LogTracer::init().expect("Failed to set logger");
    set_global_default(collector).expect("Failed to set subscriber");
    print_banner();
}

fn print_banner() {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert(env!("CARGO_PKG_NAME"));
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    info!(
        "Starting {} version: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}
