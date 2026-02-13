# Implementing a provider

This guide shows the recommended structure for an iamctl provider in Rust.

## 1) Define resource specs

Define a spec struct per resource type:

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserSpec {
  pub username: String,
  pub email: String,
}
```

## 2) Implement the `Provider` trait

A provider implements operations that the engine calls:

- `metadata()`
- `capabilities()`
- `plan()`
- `apply()`
- `validate()` (optional)
- `import()` (optional)

Focus on making `plan()` deterministic.

## 3) Register schemas

Register each spec:

```rust
let mut server = JsonRpcServer::new(MyProvider);
server.register_type_schema::<UserSpec>("user");
```

## 4) Run the server loop

The provider binary typically ends with:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_logging();
  let mut server = JsonRpcServer::new(MyProvider);
  // schema registrations...
  server.run().await
}
```

## 5) How iamctl executes it

iamctl will spawn the provider and exchange JSON-RPC messages.

From your perspective, each request becomes a call into your `Provider` implementation.

## Suggested project structure

- `src/main.rs`: wiring + server run
- `src/provider.rs`: provider implementation
- `src/resources/`: spec structs and schema helpers
- `src/api/`: client to your IAM system (HTTP SDK etc.)
