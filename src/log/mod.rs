use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use {env, figlet_rs::FIGfont, log::info};

pub fn log_init() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "ebisu=info")
    }
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_span_events(FmtSpan::FULL)
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();
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
