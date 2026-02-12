use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initializes the logging system with default settings.
/// By default, it uses the environment variable `RUST_LOG` to determine the log level.
pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
}
