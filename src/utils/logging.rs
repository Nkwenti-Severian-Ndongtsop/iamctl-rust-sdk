use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Configuration for the SDK logging system.
pub struct LogConfig {
    pub level: String,
    pub json: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            json: false,
        }
    }
}

/// Initializes the logging system with configurable settings.
/// By default, it uses the environment variable `RUST_LOG`.
pub fn init_logging() {
    init_with_config(LogConfig::default());
}

/// Initializes the logging system with a specific configuration.
pub fn init_with_config(config: LogConfig) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.level));

    let registry = tracing_subscriber::registry().with(filter);

    if config.json {
        registry.with(fmt::layer().json().with_target(true)).init();
    } else {
        registry
            .with(fmt::layer().with_target(true).compact())
            .init();
    }
}
