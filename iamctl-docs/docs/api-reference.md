# API Reference

This page provides a detailed reference for the core traits and types in the **iamctl Rust SDK**.

If you're looking for the conceptual explanation and workflow, start here:

- [Engine & Provider model](/docs/concepts/engine-provider)
- [Implementing a provider](/docs/provider-development/implementing-a-provider)

## Provider Trait

The `Provider` trait is the heart of the SDK. To build a provider, you implement this trait for your struct.

```rust
#[async_trait]
pub trait Provider: Send + Sync {
    fn metadata(&self) -> ProviderMetadata;
    fn capabilities(&self) -> ProviderCapabilities;

    async fn plan(&self, request: PlanRequest) -> anyhow::Result<PlanResponse>;
    async fn apply(&self, request: ApplyRequest) -> anyhow::Result<ApplyResponse>;

    async fn validate(&self, _request: ValidateRequest) -> anyhow::Result<ValidateResponse> {
        Ok(ValidateResponse {
            valid: true,
            errors: vec![],
        })
    }

    async fn import(&self, _request: ImportRequest) -> anyhow::Result<ImportResponse> {
        Err(anyhow::anyhow!("Import not implemented"))
    }
}
```

### Metadata & Capabilities

- **`metadata()`**: Returns basic info about your provider (name, version, author).
- **`capabilities()`**: Defines what your provider can do (which resource types it supports, if it supports import/validation).

### Core Operations

- **`plan()`**: Compares current and desired state to generate a list of changes.
- **`apply()`**: Executes the planned changes.
- **`validate()`**: Performs custom validation on resource specifications.
- **`import()`**: Allows bringing existing resources under iamctl management.

## Core Types

### Resource

Represents a single infrastructure resource managed by iamctl.

```rust
pub struct Resource {
    pub address: ResourceAddress,
    pub spec: HashMap<String, Value>,
    pub metadata: HashMap<String, String>,
}
```

### Change

Represents a specific modification to a resource.

```rust
pub struct Change {
    pub address: ResourceAddress,
    pub change_type: ChangeType,
    pub before: Option<Resource>,
    pub after: Option<Resource>,
    pub computed_fields: Vec<String>,
}
```

## Validation Framework

The SDK uses `schemars` for schema generation and `jsonschema` for validation.

### Registering Schemas

You can register schemas for your resource types in the `JsonRpcServer`:

```rust
let mut server = JsonRpcServer::new(MyProvider);
server.register_type_schema::<MyResourceSpec>("my_resource");
```

This automatically generates a JSON Schema from your Rust struct and validates all incoming requests against it.

## See also

- [Resources and changes](/docs/concepts/resources-changes)
- [State management](/docs/concepts/state)
- [Validation](/docs/concepts/validation)
- [JSON-RPC protocol (overview)](/docs/reference/json-rpc)
