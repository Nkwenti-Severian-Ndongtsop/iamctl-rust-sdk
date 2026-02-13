# Getting Started

Welcome to the **iamctl Rust SDK**! This guide will help you get started with building your first iamctl provider.

## Installation

Add the SDK to your `Cargo.toml`:

```toml
[dependencies]
iamctl-rust-sdk = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
schemars = "0.8"
```

## Core Concepts

The SDK is built around a few core concepts:

1.  **Provider Trait**: The main interface you implement to define your provider's behavior.
2.  **JSON-RPC Server**: A built-in server that handles communication between iamctl and your provider.
3.  **State Management**: Tools for managing resource state with built-in locking and security.
4.  **Schema Validation**: Automatic validation of resource specifications using JSON Schema.

## Your First Provider

Here's a minimal example of a provider that manages a simple "user" resource:

```rust
use iamctl_rust_sdk::prelude::*;
use iamctl_rust_sdk::server::JsonRpcServer;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct UserSpec {
    username: String,
    email: String,
}

struct MyProvider;

#[async_trait]
impl Provider for MyProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            name: "my-provider".to_string(),
            version: "0.1.0".to_string(),
            author: Some("Your Name".to_string()),
            ..Default::default()
        }
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            supported_resources: vec!["user".to_string()],
            can_import: true,
            can_validate: true,
        }
    }

    async fn plan(&self, request: PlanRequest) -> anyhow::Result<PlanResponse> {
        // Implementation logic for planning changes
        Ok(PlanResponse { changes: vec![] })
    }

    async fn apply(&self, request: ApplyRequest) -> anyhow::Result<ApplyResponse> {
        // Implementation logic for applying changes
        Ok(ApplyResponse {
            successful_addresses: vec![],
            failed_addresses: vec![],
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();
    let mut server = JsonRpcServer::new(MyProvider);
    server.register_type_schema::<UserSpec>("user");
    server.run().await
}
```

## Next Steps

- Explore the [API Reference](/docs/api-reference) for detailed trait and type information.
- Learn the runtime model in [Engine & Provider](/docs/concepts/engine-provider).
- Follow the guide for [Implementing a provider](/docs/provider-development/implementing-a-provider).
