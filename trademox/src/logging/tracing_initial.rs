use std::env::var;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_tracing() {
    let rust_log = var("RUST_LOG").unwrap_or_else(|_| "info".into());
    let env_filter = EnvFilter::new(rust_log);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer())
        .init();
}
