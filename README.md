# iamctl-rust-sdk

Rust SDK for building iamctl providers - the foundational SDK that enables the entire iamctl provider ecosystem.

## Overview

`iamctl-rust-sdk` is a comprehensive Rust SDK that provides all the tools, types, and protocols needed to build providers for the iamctl declarative IAM management tool.

## Features

- **Provider Trait**: Clean, async trait-based interface for implementing providers
- **JSON-RPC Protocol**: Built-in communication layer for engine-provider communication
- **Type Safety**: Strong typing with comprehensive validation
- **Error Handling**: Robust error system with actionable messages
- **Performance**: Zero-cost abstractions and efficient implementations
- **Testing**: Comprehensive testing utilities and mock providers
- **Documentation**: Complete API documentation with examples

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
iamctl-rust-sdk = "0.1.0"
```

Create a basic provider:

```rust
use iamctl_rust_sdk::prelude::*;

struct MyProvider;

#[async_trait]
impl Provider for MyProvider {
    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            name: "my-provider".to_string(),
            version: "1.0.0".to_string(),
        }
    }
    
    async fn plan(&self, request: PlanRequest) -> Result<PlanResponse> {
        // Implementation here
        Ok(PlanResponse {
            changes: vec![],
        })
    }
    
    // Implement other required methods...
}

#[tokio::main]
async fn main() -> Result<()> {
    let provider = MyProvider;
    let server = JsonRpcServer::new(provider);
    server.serve().await?;
    Ok(())
}
```

## Architecture

The SDK is organized into several key modules:

- **`provider`**: Core trait definitions and provider interfaces
- **`server`**: JSON-RPC server implementation for communication
- **`types`**: Common types and data structures
- **`validation`**: Schema validation and type safety
- **`utils`**: Utilities and helpers for provider development

## Documentation

- **API Documentation**: [docs.rs/iamctl-rust-sdk](https://docs.rs/iamctl-rust-sdk)
- **Examples**: See the `examples/` directory for complete provider implementations
- **Architecture**: See `ARCHITECTURE.md` for detailed design information
- **Roadmap**: See `todo/` directory for implementation progress

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Examples

```bash
cargo run --example basic_provider
```

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

## License

This project is dual-licensed under either:

- **MIT License** - see [LICENSE-MIT](LICENSE-MIT)
- **Apache License, Version 2.0** - see [LICENSE-APACHE](LICENSE-APACHE)

## Support

- **Issues**: [GitHub Issues](https://github.com/Nkwenti-Severian-Ndongtsop/iamctl-rust-sdk/issues)
- **Documentation**: [docs.rs](https://docs.rs/iamctl-rust-sdk)

---

*This SDK is the foundation for the entire iamctl provider ecosystem. By building providers with this SDK, you're contributing to a more automated, secure, and manageable approach to IAM infrastructure.*
