use thiserror::Error;
use crate::types::ResourceAddress;

/// Core error type for the IAMCTL SDK and providers.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(ResourceAddress),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Task joined error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// A specialized Result type for IAMCTL operations.
pub type Result<T> = std::result::Result<T, Error>;
