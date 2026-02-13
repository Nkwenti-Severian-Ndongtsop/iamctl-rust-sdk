# Introduction

**iamctl-rust-sdk** is the official Rust software development kit for building iamctl providers. It provides a high-level, type-safe framework that abstracts away the complexities of the underlying JSON-RPC protocol, state management, and schema validation.

## Why iamctl?

iamctl is a modern infrastructure-as-code engine designed for identity and access management. Unlike traditional IaC tools, iamctl focuses on the unique challenges of managing security resources, providing fine-grained control and auditing.

## SDK Features

- **Type-Safe Provider Interface**: Implement the `Provider` trait and let the SDK handle the rest.
- **Automated Validation**: Use `schemars` to automatically generate JSON Schemas from your Rust structs.
- **Secure State Storage**: Local file backend with mandatory restricted permissions and file locking.
- **Protocol abstraction**: Seamless handling of JSON-RPC over standard input/output.
- **Comprehensive Logging**: Structured logging with support for multiple formats and levels.

## Architecture Overview

The SDK acts as a bridge between the **iamctl Engine** and your **Provider Implementation**.

```text
[ iamctl Engine ] <--- JSON-RPC (stdin/stdout) ---> [ iamctl-rust-sdk ] <--- Trait Methods ---> [ Your Provider ]
```

1.  **Transport Layer**: Handles reading and writing JSON-RPC messages.
2.  **Handler Layer**: Routes incoming requests to the appropriate provider methods.
3.  **Provider Layer**: Where your custom logic for planning, applying, and validating resides.
4.  **State Layer**: Manages local caching and persistence of resource data.

## Next Steps

Ready to build? Head over to the [Getting Started](/docs/getting-started) guide to set up your development environment.

Then continue with:

- [Engine & Provider](/docs/concepts/engine-provider)
- [Implementing a provider](/docs/provider-development/implementing-a-provider)
