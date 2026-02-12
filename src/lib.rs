#![deny(unused_crate_dependencies)]
#![deny(unused_results)]
#![deny(dead_code)]
#![deny(unused_imports)]

// Force usage of dependencies that are used in submodules or for features
use anyhow as _;
use chrono as _;
use futures as _;
use jsonschema as _;
use schemars as _;
use serde_yaml as _;
use tracing as _;
use uuid as _;

#[cfg(test)]
use mockall as _;
#[cfg(test)]
use pretty_assertions as _;
#[cfg(test)]
use proptest as _;
#[cfg(test)]
use tempfile as _;
#[cfg(test)]
use tokio_test as _;
#[cfg(test)]
use criterion as _;
#[cfg(test)]
use insta as _;

pub mod provider;
pub mod server;
pub mod types;
pub mod utils;
pub mod validation;

pub mod prelude {
    pub use crate::provider::{
        ApplyRequest, ApplyResponse, PlanRequest, PlanResponse, Provider, ProviderCapabilities,
        ProviderMetadata,
    };
    pub use crate::types::{Change, ChangeType, Resource, ResourceAddress};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
