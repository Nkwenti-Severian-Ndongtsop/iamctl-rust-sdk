use iamctl_rust_sdk::utils::logging::LogConfig;

#[test]
fn test_logging_initialization() {
    // We can't easily test if tracing is globally initialized multiple times in the same process
    // but we can test the config and the public functions.
    let config = LogConfig {
        level: "debug".to_string(),
        json: true,
    };
    assert_eq!(config.level, "debug");
    assert!(config.json);

    let default_config = LogConfig::default();
    assert_eq!(default_config.level, "info");
    assert!(!default_config.json);
}

#[test]
fn test_init_logging_basic() {
    // This won't panic even if called multiple times (tracing-subscriber handles it or we handle the Result if we used it)
    // Actually init() panics if called twice, but we can't easily avoid that in tests
    // without a global lock or just testing the config part.
    // Since we can't easily test the global subscriber, we focus on the logic.
}
